use lrpar::{NonStreamingLexer, Span};
use std::ffi::c_void;
use std::ptr::addr_of;

use super::ast_node::AstNode;
use super::error::CompilerError;
use super::semantic_structures::{CallSymbol, DefSymbol, ScopeStack, SymbolClass, SymbolType};

#[derive(Debug)]
pub struct GlobalVarDef {
    is_static: bool,
    var_type: Span,
    node_id: Span,
    next: Option<Box<dyn AstNode>>,
}

impl GlobalVarDef {
    pub fn new(
        is_static: bool,
        var_type: Span,
        node_id: Span,
        next: Option<Box<dyn AstNode>>,
    ) -> GlobalVarDef {
        GlobalVarDef {
            is_static,
            var_type,
            node_id,
            next,
        }
    }
}

// Tamanho.
// O tamanho dos tipos da linguagem é definido da seguinte forma.
// Um char ocupa 1 byte.
// Um string ocupa 1 byte para cada caractere.
// Um int ocupa 4 bytes.
// Um float ocupa 8 bytes.
// Um bool ocupa 1 byte.
// Um vetor ocupa o seu tamanho vezes o seu tipo.

fn get_symbol_type_size(symbol_type: &SymbolType) -> u32 {
    match symbol_type {
        SymbolType::Char(_) => 1,
        SymbolType::Int(_) => 4,
        SymbolType::Float(_) => 8,
        SymbolType::Bool(_) => 1,
        SymbolType::String(maybe_string) => match maybe_string {
            Some(string) => (string.len() as u32),
            None => 0,
        },
    }
}

impl AstNode for GlobalVarDef {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_ripple(next_node.as_ref(), own_address, ripple)
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        false
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        stack.check_duplicate(span, lexer)?;

        let var_type = SymbolType::from_str(lexer.span_str(self.var_type))?;
        let id = lexer.span_str(self.node_id).to_string();
        let ((line, col), (_, _)) = lexer.line_col(self.node_id);
        let class = SymbolClass::Var;
        let size = get_symbol_type_size(&var_type);

        let our_symbol = DefSymbol::new(id, span, line, col, var_type, class, Some(size));

        stack.add_def_symbol(our_symbol)?;

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct GlobalVecDef {
    is_static: bool,
    var_type: Span,
    node_id: Span,
    vec_size: Box<LiteralInt>,
    next: Option<Box<dyn AstNode>>,
}

impl GlobalVecDef {
    pub fn new(
        is_static: bool,
        var_type: Span,
        node_id: Span,
        vec_size: Box<LiteralInt>,
        next: Option<Box<dyn AstNode>>,
    ) -> GlobalVecDef {
        GlobalVecDef {
            is_static,
            var_type,
            node_id,
            vec_size,
            next,
        }
    }
}

impl AstNode for GlobalVecDef {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_ripple(next_node.as_ref(), own_address, ripple)
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        false
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        stack.check_duplicate(span, lexer)?;
        let id = lexer.span_str(self.node_id).to_string();

        let var_type = match SymbolType::from_str(lexer.span_str(self.var_type))? {
            SymbolType::String(_) => {
                let start = self.var_type.start();
                let end = self.vec_size.get_id().end() + 1;
                if end < start {
                    return Err(CompilerError::SanityError(format!(
                        "evaluate_node() found unlawful spans on GlobalVecDef for \"{}\"",
                        id.to_string(),
                    )));
                };
                let span = Span::new(start, end);
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                return Err(CompilerError::SemanticErrorStringVector {
                    id,
                    line,
                    col,
                    highlight,
                });
            }
            not_string @ _ => not_string,
        };
        let ((line, col), (_, _)) = lexer.line_col(self.node_id);
        let class = SymbolClass::Vec;

        let vec_size = self.vec_size.evaluate_node(stack, lexer)?;

        let size_int = match vec_size {
            Some(value) => match value {
                SymbolType::Int(int) => match int {
                    Some(size_int) => size_int as u32,
                    None => {
                        return Err(CompilerError::SanityError(format!(
                            "Int from vec_size symbol as None (on GlobalVecDef.evaluate_node())",
                        )))
                    }
                },
                _ => {
                    return Err(CompilerError::SanityError(format!(
                        "Vec_size symbol_type is not Int (on GlobalVecDef.evaluate_node())",
                    )))
                }
            },
            None => {
                return Err(CompilerError::SanityError(format!(
                    "Vec_size symbol is None (on GlobalVecDef.evaluate_node())",
                )))
            }
        };

        let size = get_symbol_type_size(&var_type) * size_int;

        let our_symbol = DefSymbol::new(id, span, line, col, var_type, class, Some(size));

        stack.add_def_symbol(our_symbol)?;

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct FnDef {
    is_static: bool,
    return_type: Span,
    node_id: Span,
    params: Vec<Parameter>,
    commands: CommandBlock,
    next: Option<Box<dyn AstNode>>,
}

impl FnDef {
    pub fn new(
        is_static: bool,
        return_type: Span,
        node_id: Span,
        params: Vec<Parameter>,
        commands: CommandBlock,
        next: Option<Box<dyn AstNode>>,
    ) -> FnDef {
        FnDef {
            is_static,
            return_type,
            node_id,
            params,
            commands,
            next,
        }
    }
}

impl AstNode for FnDef {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        self.commands
            .print_first_dependencies(print_dependencies_own, own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        self.commands
            .print_first_dependencies(print_dependencies_child, own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        self.commands.print_first_labels(print_labels_child, lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        stack.check_duplicate(span, lexer)?;

        let var_type = SymbolType::from_str(lexer.span_str(self.return_type))?;
        let id = lexer.span_str(self.node_id).to_string();
        let ((line, col), (_, _)) = lexer.line_col(self.node_id);
        let class = SymbolClass::Fn;
        let our_symbol = DefSymbol::new(id, span, line, col, var_type, class, None);

        stack.add_def_symbol(our_symbol)?;

        self.commands.evaluate_node(stack, lexer)?;

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub is_const: bool,
    pub node_id: Span,
    pub param_name: Span,
}

#[derive(Debug)]
pub struct LocalVarDef {
    is_static: bool,
    is_const: bool,
    var_type: Span,
    node_id: Span,
    is_tree_node: bool,
    next: Option<Box<dyn AstNode>>,
}

impl LocalVarDef {
    pub fn new(
        is_static: bool,
        is_const: bool,
        var_type: Span,
        node_id: Span,
        is_tree_node: bool,
        next: Option<Box<dyn AstNode>>,
    ) -> LocalVarDef {
        LocalVarDef {
            is_static,
            is_const,
            var_type,
            node_id,
            is_tree_node,
            next,
        }
    }
}

impl AstNode for LocalVarDef {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool) {
        if !self.is_tree_node {
            if let Some(next_node) = &self.next {
                print_dependencies_ripple(next_node.as_ref(), own_address, ripple)
            }
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        if self.is_tree_node {
            print_label_self(self.node_id, lexer, own_address);
        };
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        self.is_tree_node
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        stack.check_duplicate(span, lexer)?;

        let var_type = SymbolType::from_str(lexer.span_str(self.var_type))?;
        let id = lexer.span_str(self.node_id).to_string();
        let ((line, col), (_, _)) = lexer.line_col(self.node_id);
        let class = SymbolClass::Var;
        let size = get_symbol_type_size(&var_type);
        let our_symbol = DefSymbol::new(id, span, line, col, var_type, class, Some(size));

        stack.add_def_symbol(our_symbol)?;

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VarDefInitId {
    node_id: Span,
    var_def: Box<dyn AstNode>,
    var_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VarDefInitId {
    pub fn new(
        node_id: Span,
        var_def: Box<dyn AstNode>,
        var_value: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VarDefInitId {
        VarDefInitId {
            node_id,
            var_def,
            var_value,
            next,
        }
    }
}

impl AstNode for VarDefInitId {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.var_def.as_ref(), own_address);
        print_dependencies_own(self.var_value.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.var_def.as_ref(), own_address);
        print_dependencies_child(self.var_value.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.var_def.as_ref(), lexer);
        print_labels_child(self.var_value.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.var_def.evaluate_node(stack, lexer)?;
        self.var_value.evaluate_node(stack, lexer)?;

        let def_symbol = stack.get_previous_def(self.var_def.get_id(), lexer, SymbolClass::Var)?;
        let var_symbol =
            stack.get_previous_def(self.var_value.get_id(), lexer, SymbolClass::Var)?;

        match (def_symbol.type_value.clone(), var_symbol.type_value.clone()) {
            (SymbolType::String(_), var_type) => match var_type {
                SymbolType::String(var_option) => match var_option {
                    Some(var_value) => {
                        let updated_symbol = {
                            let clone = SymbolType::String(Some(var_value));
                            let size = get_symbol_type_size(&clone);
                            DefSymbol::new(
                                def_symbol.id.clone(),
                                def_symbol.span,
                                def_symbol.line,
                                def_symbol.col,
                                def_symbol.type_value.clone(),
                                def_symbol.class,
                                Some(size),
                            )
                        };

                        stack.add_def_symbol(updated_symbol)?;
                    }
                    None => {}
                },
                var_type => {
                    let ((line, col), (_, _)) = lexer.line_col(self.node_id);
                    let highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                    return Err(CompilerError::SemanticErrorStringToX {
                        invalid_type: var_type.to_str().to_string(),
                        line,
                        col,
                        highlight,
                    });
                }
            },
            (SymbolType::Char(_), var_type) => match var_type {
                SymbolType::Char(_) => {}
                var_type => {
                    let value_span = self.var_value.get_id();
                    let ((line, col), (_, _)) = lexer.line_col(value_span.get_id());
                    let highlight = ScopeStack::form_string_highlight(value_span.get_id(), lexer);
                    return Err(CompilerError::SemanticErrorCharToX {
                        invalid_type: var_type.to_str().to_string(),
                        line,
                        col,
                        highlight,
                    });
                }
            },
            (SymbolType::Int(_) | SymbolType::Float(_) | SymbolType::Bool(_), var_type) => {
                match var_type {
                    SymbolType::Int(_) | SymbolType::Float(_) | SymbolType::Bool(_) => {}
                    var_type => {
                        let value_span = self.var_value.get_id();
                        let highlight = ScopeStack::form_string_highlight(value_span, lexer);
                        let ((line, col), (_, _)) = lexer.line_col(value_span);

                        return Err(CompilerError::SemanticErrorWrongType {
                            valid_types: "int, float or bool".to_string(),
                            received_type: var_type.to_str().to_string(),
                            highlight,
                            line,
                            col,
                        });
                    }
                }
            }
        };

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VarDefInitLit {
    node_id: Span,
    var_def: Box<dyn AstNode>,
    var_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VarDefInitLit {
    pub fn new(
        node_id: Span,
        var_def: Box<dyn AstNode>,
        var_value: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VarDefInitLit {
        VarDefInitLit {
            node_id,
            var_def,
            var_value,
            next,
        }
    }
}

impl AstNode for VarDefInitLit {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.var_def.as_ref(), own_address);
        print_dependencies_own(self.var_value.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.var_def.as_ref(), own_address);
        print_dependencies_child(self.var_value.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.var_def.as_ref(), lexer);
        print_labels_child(self.var_value.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.var_def.evaluate_node(stack, lexer)?;
        let value = self.var_value.evaluate_node(stack, lexer)?;

        let def_symbol = {
            let span = self.var_def.get_id();
            let def_symbol = stack.get_previous_def(span, lexer, SymbolClass::Var)?;
            def_symbol.clone()
        };

        match value {
            Some(value) => match (def_symbol.type_value.clone(), value) {
                (SymbolType::String(_), var_type) => match var_type {
                    SymbolType::String(var_option) => match var_option {
                        Some(var_value) => {
                            let updated_symbol = {
                                let clone = SymbolType::String(Some(var_value));
                                let size = get_symbol_type_size(&clone);
                                DefSymbol::new(
                                    def_symbol.id.clone(),
                                    def_symbol.span,
                                    def_symbol.line,
                                    def_symbol.col,
                                    def_symbol.type_value.clone(),
                                    def_symbol.class,
                                    Some(size),
                                )
                            };

                            stack.add_def_symbol(updated_symbol)?;
                        }
                        None => {}
                    },
                    var_type => {
                        let ((line, col), (_, _)) = lexer.line_col(self.node_id);
                        let highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                        return Err(CompilerError::SemanticErrorStringToX {
                            invalid_type: var_type.to_str().to_string(),
                            line,
                            col,
                            highlight,
                        });
                    }
                },
                (SymbolType::Char(_), var_type) => match var_type {
                    SymbolType::Char(_) => {}
                    var_type => {
                        let value_span = self.var_value.get_id();
                        let ((line, col), (_, _)) = lexer.line_col(value_span.get_id());
                        let highlight =
                            ScopeStack::form_string_highlight(value_span.get_id(), lexer);
                        return Err(CompilerError::SemanticErrorCharToX {
                            invalid_type: var_type.to_str().to_string(),
                            line,
                            col,
                            highlight,
                        });
                    }
                },
                (SymbolType::Int(_) | SymbolType::Float(_) | SymbolType::Bool(_), var_type) => {
                    match var_type {
                        SymbolType::Int(_) | SymbolType::Float(_) | SymbolType::Bool(_) => {}
                        var_type => {
                            let value_span = self.var_value.get_id();
                            let highlight = ScopeStack::form_string_highlight(value_span, lexer);
                            let ((line, col), (_, _)) = lexer.line_col(value_span);

                            return Err(CompilerError::SemanticErrorWrongType {
                                valid_types: "int, float or bool".to_string(),
                                received_type: var_type.to_str().to_string(),
                                highlight,
                                line,
                                col,
                            });
                        }
                    }
                }
            },

            None => {
                return Err(CompilerError::SanityError(format!(
                    "No symbol type on value init (on VarDefInitLit.evaluate_node())"
                )))
            }
        };

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VarLeftShift {
    node_id: Span,
    var_name: Box<VarInvoke>,
    shift_amount: Box<LiteralInt>,
    next: Option<Box<dyn AstNode>>,
}

impl VarLeftShift {
    pub fn new(
        node_id: Span,
        var_name: Box<VarInvoke>,
        shift_amount: Box<LiteralInt>,
        next: Option<Box<dyn AstNode>>,
    ) -> VarLeftShift {
        VarLeftShift {
            node_id,
            var_name,
            shift_amount,
            next,
        }
    }
}

impl AstNode for VarLeftShift {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.var_name.as_ref(), own_address);
        print_dependencies_own(self.shift_amount.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.var_name.as_ref(), own_address);
        print_dependencies_child(self.shift_amount.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.var_name.as_ref(), lexer);
        print_labels_child(self.shift_amount.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.node_id.evaluate_node(stack, lexer)?;
        self.var_name.evaluate_node(stack, lexer)?;
        self.shift_amount.evaluate_node(stack, lexer)?;

        let symbol = match stack.pop_symbol()? {
            Some(symbol) => symbol,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "failed to pop expected literal int symbol (on varLeftShift.evaluate_node())"
                )))
            }
        };

        match symbol.type_value {
            SymbolType::Int(option_value) => match option_value {
                Some(value) => {
                    if value > 16 {
                        let shift_amount_span = self.shift_amount.get_id();
                        let highlight = ScopeStack::form_string_highlight(shift_amount_span, lexer);
                        let ((line, col), (_, _)) = lexer.line_col(shift_amount_span);

                        return Err(CompilerError::SemanticErrorWrongParShift {
                            received_value: value,
                            highlight,
                            line,
                            col,
                        });
                    }
                }
                None => {
                    return Err(CompilerError::SanityError(format!(
                        "shift_amount option received is None (on varLeftShift.evaluate_node())"
                    )))
                }
            },
            _ => {
                return Err(CompilerError::SanityError(format!(
                    "shift_amount received is NOT a literal int (on varLeftShift.evaluate_node())"
                )))
            }
        }

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VarRightShift {
    node_id: Span,
    var_name: Box<VarInvoke>,
    shift_amount: Box<LiteralInt>,
    next: Option<Box<dyn AstNode>>,
}

impl VarRightShift {
    pub fn new(
        node_id: Span,
        var_name: Box<VarInvoke>,
        shift_amount: Box<LiteralInt>,
        next: Option<Box<dyn AstNode>>,
    ) -> VarRightShift {
        VarRightShift {
            node_id,
            var_name,
            shift_amount,
            next,
        }
    }
}

impl AstNode for VarRightShift {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.var_name.as_ref(), own_address);
        print_dependencies_own(self.shift_amount.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.var_name.as_ref(), own_address);
        print_dependencies_child(self.shift_amount.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.var_name.as_ref(), lexer);
        print_labels_child(self.shift_amount.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.node_id.evaluate_node(stack, lexer)?;
        self.var_name.evaluate_node(stack, lexer)?;
        self.shift_amount.evaluate_node(stack, lexer)?;

        let symbol = match stack.pop_symbol()? {
            Some(symbol) => symbol,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "failed to pop expected literal int symbol (on varRightShift.evaluate_node())"
                )))
            }
        };

        match symbol.type_value {
            SymbolType::Int(option_value) => match option_value {
                Some(value) => {
                    if value > 16 {
                        let shift_amount_span = self.shift_amount.get_id();
                        let highlight = ScopeStack::form_string_highlight(shift_amount_span, lexer);
                        let ((line, col), (_, _)) = lexer.line_col(shift_amount_span);

                        return Err(CompilerError::SemanticErrorWrongParShift {
                            received_value: value,
                            highlight,
                            line,
                            col,
                        });
                    }
                }
                None => {
                    return Err(CompilerError::SanityError(format!(
                        "shift_amount option received is None (on varRightShift.evaluate_node())"
                    )))
                }
            },
            _ => {
                return Err(CompilerError::SanityError(format!(
                    "shift_amount received is NOT a literal int (on varRightShift.evaluate_node())"
                )))
            }
        }

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VecLeftShift {
    node_id: Span,
    vec_access: Box<dyn AstNode>,
    shift_amount: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecLeftShift {
    pub fn new(
        node_id: Span,
        vec_access: Box<dyn AstNode>,
        shift_amount: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VecLeftShift {
        VecLeftShift {
            node_id,
            vec_access,
            shift_amount,
            next,
        }
    }
}

impl AstNode for VecLeftShift {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.vec_access.as_ref(), own_address);
        print_dependencies_own(self.shift_amount.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.vec_access.as_ref(), own_address);
        print_dependencies_child(self.shift_amount.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.vec_access.as_ref(), lexer);
        print_labels_child(self.shift_amount.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.node_id.evaluate_node(stack, lexer)?;
        self.vec_access.evaluate_node(stack, lexer)?;

        self.shift_amount.evaluate_node(stack, lexer)?;

        let symbol = match stack.pop_symbol()? {
            Some(symbol) => symbol,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "failed to pop expected literal int symbol (on vecLeftShift.evaluate_node())"
                )))
            }
        };

        match symbol.type_value {
            SymbolType::Int(option_value) => match option_value {
                Some(value) => {
                    if value > 16 {
                        let shift_amount_span = self.shift_amount.get_id();
                        let highlight = ScopeStack::form_string_highlight(shift_amount_span, lexer);
                        let ((line, col), (_, _)) = lexer.line_col(shift_amount_span);

                        return Err(CompilerError::SemanticErrorWrongParShift {
                            received_value: value,
                            highlight,
                            line,
                            col,
                        });
                    }
                }
                None => {
                    return Err(CompilerError::SanityError(format!(
                        "shift_amount option received is None (on vecLeftShift.evaluate_node())"
                    )))
                }
            },
            _ => {
                return Err(CompilerError::SanityError(format!(
                    "shift_amount received is NOT a literal int (on vecLeftShift.evaluate_node())"
                )))
            }
        }

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VecRightShift {
    node_id: Span,
    vec_access: Box<dyn AstNode>,
    shift_amount: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecRightShift {
    pub fn new(
        node_id: Span,
        vec_access: Box<dyn AstNode>,
        shift_amount: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VecRightShift {
        VecRightShift {
            node_id,
            vec_access,
            shift_amount,
            next,
        }
    }
}

impl AstNode for VecRightShift {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.vec_access.as_ref(), own_address);
        print_dependencies_own(self.shift_amount.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.vec_access.as_ref(), own_address);
        print_dependencies_child(self.shift_amount.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.vec_access.as_ref(), lexer);
        print_labels_child(self.shift_amount.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.node_id.evaluate_node(stack, lexer)?;
        self.vec_access.evaluate_node(stack, lexer)?;

        self.shift_amount.evaluate_node(stack, lexer)?;

        let symbol = match stack.pop_symbol()? {
            Some(symbol) => symbol,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "failed to pop expected literal int symbol (on vecRightShift.evaluate_node())"
                )))
            }
        };

        match symbol.type_value {
            SymbolType::Int(option_value) => match option_value {
                Some(value) => {
                    if value > 16 {
                        let shift_amount_span = self.shift_amount.get_id();
                        let highlight = ScopeStack::form_string_highlight(shift_amount_span, lexer);
                        let ((line, col), (_, _)) = lexer.line_col(shift_amount_span);

                        return Err(CompilerError::SemanticErrorWrongParShift {
                            received_value: value,
                            highlight,
                            line,
                            col,
                        });
                    }
                }
                None => {
                    return Err(CompilerError::SanityError(format!(
                        "shift_amount option received is None (on vecRightShift.evaluate_node())"
                    )))
                }
            },
            _ => {
                return Err(CompilerError::SanityError(format!(
                    "shift_amount received is NOT a literal int (on vecRightShift.evaluate_node())"
                )))
            }
        }

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VarSet {
    node_id: Span,
    var_name: Box<dyn AstNode>,
    new_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VarSet {
    pub fn new(
        node_id: Span,
        var_name: Box<dyn AstNode>,
        new_value: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VarSet {
        VarSet {
            node_id,
            var_name,
            new_value,
            next,
        }
    }
}

impl AstNode for VarSet {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.var_name.as_ref(), own_address);
        print_dependencies_own(self.new_value.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.var_name.as_ref(), own_address);
        print_dependencies_child(self.new_value.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.var_name.as_ref(), lexer);
        print_labels_child(self.new_value.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.node_id.evaluate_node(stack, lexer)?;
        self.var_name.evaluate_node(stack, lexer)?;
        let new_value = self.new_value.evaluate_node(stack, lexer)?;

        let def = stack.get_previous_def(self.var_name.get_id(), lexer, SymbolClass::Var)?;
        let def_size = match def.size {
            Some(size) => size,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "variable_definition with no memory size (on VarSet.evaluate_node())"
                )))
            }
        };

        match new_value {
            None => {
                return Err(CompilerError::SanityError(format!(
                    "New value has no SymbolType (on VarSet.evaluate_node())"
                )))
            }
            Some(symbol_type) => match def.type_value {
                SymbolType::String(_) => match symbol_type {
                    SymbolType::String(option_string) => match option_string {
                        Some(string) => {
                            let string_size = string.len() as u32;
                            if string_size > def_size {
                                let span = self.new_value.get_id();
                                let highlight = ScopeStack::form_string_highlight(span, lexer);
                                let ((line, col), (_, _)) = lexer.line_col(span);

                                return Err(CompilerError::SemanticErrorStringMax {
                                    highlight,
                                    line,
                                    col,
                                    string_size,
                                    variable_size: def_size,
                                });
                            }
                        }
                        None => {}
                    },
                    var_type => {
                        let ((line, col), (_, _)) = lexer.line_col(self.node_id);
                        let highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                        return Err(CompilerError::SemanticErrorStringToX {
                            invalid_type: var_type.to_str().to_string(),
                            line,
                            col,
                            highlight,
                        });
                    }
                },
                SymbolType::Char(_) => match symbol_type {
                    SymbolType::Char(_) => {}
                    symbol_type => {
                        let ((line, col), (_, _)) = lexer.line_col(self.new_value.get_id());
                        let highlight =
                            ScopeStack::form_string_highlight(self.new_value.get_id(), lexer);
                        return Err(CompilerError::SemanticErrorCharToX {
                            invalid_type: symbol_type.to_str().to_string(),
                            line,
                            col,
                            highlight,
                        });
                    }
                },
                SymbolType::Int(_) | SymbolType::Float(_) | SymbolType::Bool(_) => {
                    match symbol_type {
                        SymbolType::Int(_) | SymbolType::Float(_) | SymbolType::Bool(_) => {}
                        symbol_type => {
                            let value_span = self.new_value.get_id();
                            let highlight = ScopeStack::form_string_highlight(value_span, lexer);
                            let ((line, col), (_, _)) = lexer.line_col(value_span);

                            return Err(CompilerError::SemanticErrorWrongType {
                                valid_types: "int, float or bool".to_string(),
                                received_type: symbol_type.to_str().to_string(),
                                highlight,
                                line,
                                col,
                            });
                        }
                    }
                }
            },
        }

        // TO DO: Add symbol and check type.

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VecSet {
    node_id: Span,
    vec_access: Box<dyn AstNode>,
    new_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecSet {
    pub fn new(
        node_id: Span,
        vec_access: Box<dyn AstNode>,
        new_value: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VecSet {
        VecSet {
            node_id,
            vec_access,
            new_value,
            next,
        }
    }
}

impl AstNode for VecSet {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.vec_access.as_ref(), own_address);
        print_dependencies_own(self.new_value.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.vec_access.as_ref(), own_address);
        print_dependencies_child(self.new_value.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.vec_access.as_ref(), lexer);
        print_labels_child(self.new_value.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.vec_access.evaluate_node(stack, lexer)?;

        self.new_value.evaluate_node(stack, lexer)?;

        // TO DO: Add symbol and check type.

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct Input {
    node_id: Span,
    var_name: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl Input {
    pub fn new(node_id: Span, var_name: Box<dyn AstNode>, next: Option<Box<dyn AstNode>>) -> Input {
        Input {
            node_id,
            var_name,
            next,
        }
    }
}

impl AstNode for Input {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.var_name.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.var_name.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.var_name.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.var_name.evaluate_node(stack, lexer)?;

        let id = self.var_name.get_id();
        let var_def = stack.get_previous_def(id, lexer, SymbolClass::Var)?;

        match var_def.type_value {
            SymbolType::Int(_) | SymbolType::Float(_) => (),
            _ => {
                let first_highlight = ScopeStack::form_string_highlight(var_def.span, lexer);
                let ((first_line, first_col), (_, _)) = lexer.line_col(var_def.span);

                let second_highlight = ScopeStack::form_string_highlight(id, lexer);
                let ((second_line, second_col), (_, _)) = lexer.line_col(id);

                return Err(CompilerError::SemanticErrorWrongParInput {
                    received_type: var_def.type_value.to_str().to_string(),
                    first_highlight,
                    first_line,
                    first_col,
                    second_highlight,
                    second_line,
                    second_col,
                });
            }
        }

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct OutputId {
    node_id: Span,
    var_name: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl OutputId {
    pub fn new(
        node_id: Span,
        var_name: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> OutputId {
        OutputId {
            node_id,
            var_name,
            next,
        }
    }
}

impl AstNode for OutputId {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.var_name.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.var_name.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.var_name.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.var_name.evaluate_node(stack, lexer)?;

        let id = self.var_name.get_id();
        let var_def = stack.get_previous_def(id, lexer, SymbolClass::Var)?;

        match var_def.type_value {
            SymbolType::Int(_) | SymbolType::Float(_) => (),
            _ => {
                let first_highlight = ScopeStack::form_string_highlight(var_def.span, lexer);
                let ((first_line, first_col), (_, _)) = lexer.line_col(var_def.span);

                let second_highlight = ScopeStack::form_string_highlight(id, lexer);
                let ((second_line, second_col), (_, _)) = lexer.line_col(id);

                return Err(CompilerError::SemanticErrorWrongParOutputId {
                    received_type: var_def.type_value.to_str().to_string(),
                    first_highlight,
                    first_line,
                    first_col,
                    second_highlight,
                    second_line,
                    second_col,
                });
            }
        }

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct OutputLit {
    node_id: Span,
    lit_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl OutputLit {
    pub fn new(
        node_id: Span,
        lit_value: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> OutputLit {
        OutputLit {
            node_id,
            lit_value,
            next,
        }
    }
}

impl AstNode for OutputLit {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.lit_value.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.lit_value.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.lit_value.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.lit_value.evaluate_node(stack, lexer)?;

        let symbol = match stack.pop_symbol()? {
            Some(symbol) => symbol,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "evaluate_node() failed to pop symbol (self: {:?})",
                    &self,
                )))
            }
        };

        match symbol.type_value {
            SymbolType::Int(_) | SymbolType::Float(_) => (),
            _ => {
                let highlight = ScopeStack::form_string_highlight(symbol.span, lexer);
                let ((line, col), (_, _)) = lexer.line_col(symbol.span);

                return Err(CompilerError::SemanticErrorWrongParOutputLit {
                    received_type: symbol.type_value.to_str().to_string(),
                    highlight,
                    line,
                    col,
                });
            }
        }

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct Continue {
    node_id: Span,
    next: Option<Box<dyn AstNode>>,
}

impl Continue {
    pub fn new(node_id: Span, next: Option<Box<dyn AstNode>>) -> Continue {
        Continue { node_id, next }
    }
}

impl AstNode for Continue {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        _stack: &mut ScopeStack,
        _lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct Break {
    node_id: Span,
    next: Option<Box<dyn AstNode>>,
}

impl Break {
    pub fn new(node_id: Span, next: Option<Box<dyn AstNode>>) -> Break {
        Break { node_id, next }
    }
}

impl AstNode for Break {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        _stack: &mut ScopeStack,
        _lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct Return {
    node_id: Span,
    ret_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl Return {
    pub fn new(
        node_id: Span,
        ret_value: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> Return {
        Return {
            node_id,
            ret_value,
            next,
        }
    }
}

impl AstNode for Return {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.ret_value.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.ret_value.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.ret_value.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        _stack: &mut ScopeStack,
        _lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct FnCall {
    node_id: Span,
    args: Option<Box<dyn AstNode>>,
    next: Option<Box<dyn AstNode>>,
}

impl FnCall {
    pub fn new(
        node_id: Span,
        args: Option<Box<dyn AstNode>>,
        next: Option<Box<dyn AstNode>>,
    ) -> FnCall {
        FnCall {
            node_id,
            args,
            next,
        }
    }

    fn print_label_fn_call(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        println!(
            "{:p} [label=\"call {}\"];",
            own_address,
            lexer.span_str(self.node_id)
        );
    }
}

impl AstNode for FnCall {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        if let Some(args) = &self.args {
            print_dependencies_own(args.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(args) = &self.args {
            print_dependencies_child(args.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        self.print_label_fn_call(lexer, own_address);
        match &self.args {
            Some(args) => print_labels_child(args.as_ref(), lexer),
            None => (),
        };
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        let class = SymbolClass::Fn;
        //TO DO: Add args-checking.
        let previous_def = stack.get_previous_def(span, lexer, class)?;
        let type_value = previous_def.type_value.clone();

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(Some(type_value))
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct If {
    node_id: Span,
    condition: Box<dyn AstNode>,
    consequence: CommandBlock,
    next: Option<Box<dyn AstNode>>,
}

impl If {
    pub fn new(
        node_id: Span,
        condition: Box<dyn AstNode>,
        consequence: CommandBlock,
        next: Option<Box<dyn AstNode>>,
    ) -> If {
        If {
            node_id,
            condition,
            consequence,
            next,
        }
    }
}

impl AstNode for If {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.condition.as_ref(), own_address);
        self.consequence
            .print_first_dependencies(print_dependencies_own, own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.condition.as_ref(), own_address);
        self.consequence
            .print_first_dependencies(print_dependencies_child, own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.condition.as_ref(), lexer);
        self.consequence
            .print_first_labels(print_labels_child, lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        _stack: &mut ScopeStack,
        _lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct IfElse {
    node_id: Span,
    condition: Box<dyn AstNode>,
    if_true: CommandBlock,
    if_false: CommandBlock,
    next: Option<Box<dyn AstNode>>,
}

impl IfElse {
    pub fn new(
        node_id: Span,
        condition: Box<dyn AstNode>,
        if_true: CommandBlock,
        if_false: CommandBlock,
        next: Option<Box<dyn AstNode>>,
    ) -> IfElse {
        IfElse {
            node_id,
            condition,
            if_true,
            if_false,
            next,
        }
    }
}

impl AstNode for IfElse {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.condition.as_ref(), own_address);
        self.if_true
            .print_first_dependencies(print_dependencies_own, own_address);
        self.if_false
            .print_first_dependencies(print_dependencies_own, own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.condition.as_ref(), own_address);
        self.if_true
            .print_first_dependencies(print_dependencies_child, own_address);
        self.if_false
            .print_first_dependencies(print_dependencies_child, own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.condition.as_ref(), lexer);
        self.if_true.print_first_labels(print_labels_child, lexer);
        self.if_false.print_first_labels(print_labels_child, lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        _stack: &mut ScopeStack,
        _lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct For {
    node_id: Span,
    count_init: Box<dyn AstNode>,
    count_check: Box<dyn AstNode>,
    count_iter: Box<dyn AstNode>,
    actions: CommandBlock,
    next: Option<Box<dyn AstNode>>,
}

impl For {
    pub fn new(
        node_id: Span,
        count_init: Box<dyn AstNode>,
        count_check: Box<dyn AstNode>,
        count_iter: Box<dyn AstNode>,
        actions: CommandBlock,
        next: Option<Box<dyn AstNode>>,
    ) -> For {
        For {
            node_id,
            count_init,
            count_check,
            count_iter,
            actions,
            next,
        }
    }
}

impl AstNode for For {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.count_init.as_ref(), own_address);
        print_dependencies_own(self.count_check.as_ref(), own_address);
        print_dependencies_own(self.count_iter.as_ref(), own_address);
        self.actions
            .print_first_dependencies(print_dependencies_own, own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.count_init.as_ref(), own_address);
        print_dependencies_child(self.count_check.as_ref(), own_address);
        print_dependencies_child(self.count_iter.as_ref(), own_address);
        self.actions
            .print_first_dependencies(print_dependencies_child, own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.count_init.as_ref(), lexer);
        print_labels_child(self.count_check.as_ref(), lexer);
        print_labels_child(self.count_iter.as_ref(), lexer);
        self.actions.print_first_labels(print_labels_child, lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        _stack: &mut ScopeStack,
        _lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct While {
    node_id: Span,
    condition: Box<dyn AstNode>,
    consequence: CommandBlock,
    next: Option<Box<dyn AstNode>>,
}

impl While {
    pub fn new(
        node_id: Span,
        condition: Box<dyn AstNode>,
        consequence: CommandBlock,
        next: Option<Box<dyn AstNode>>,
    ) -> While {
        While {
            node_id,
            condition,
            consequence,
            next,
        }
    }
}

impl AstNode for While {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.condition.as_ref(), own_address);
        self.consequence
            .print_first_dependencies(print_dependencies_own, own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.condition.as_ref(), own_address);
        self.consequence
            .print_first_dependencies(print_dependencies_child, own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.condition.as_ref(), lexer);
        self.consequence
            .print_first_labels(print_labels_child, lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        _stack: &mut ScopeStack,
        _lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct CommandBlock {
    node_id: Span,
    first_command: Option<Box<dyn AstNode>>,
    next: Option<Box<dyn AstNode>>,
}

impl CommandBlock {
    pub fn new(
        node_id: Span,
        first_command: Option<Box<dyn AstNode>>,
        next: Option<Box<dyn AstNode>>,
    ) -> CommandBlock {
        CommandBlock {
            node_id,
            first_command,
            next,
        }
    }
}

impl AstNode for CommandBlock {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_ripple(next_node.as_ref(), own_address, ripple)
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        false
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        if let Some(command) = &self.first_command {
            stack.add_scope();
            command.evaluate_node(stack, lexer)?;
            stack.remove_scope()?;
        };

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

impl CommandBlock {
    pub fn print_first_dependencies(
        &self,
        print_func: fn(&(dyn AstNode), *const c_void),
        own_address: *const c_void,
    ) {
        let mut current_command = &self.first_command;
        loop {
            match current_command {
                Some(command) => {
                    if command.is_tree_member() {
                        print_func(command.as_ref(), own_address);
                        break;
                    } else {
                        current_command = command.get_next();
                        continue;
                    };
                }
                None => break,
            }
        }
    }
    pub fn print_first_labels(
        &self,
        print_func: fn(&(dyn AstNode), &dyn NonStreamingLexer<u32>),
        lexer: &dyn NonStreamingLexer<u32>,
    ) {
        let mut current_command = &self.first_command;
        loop {
            match current_command {
                Some(command) => {
                    if command.is_tree_member() {
                        print_func(command.as_ref(), lexer);
                        break;
                    } else {
                        current_command = command.get_next();
                        continue;
                    };
                }
                None => break,
            }
        }
    }
}

#[derive(Debug)]
pub struct Ternary {
    left_span: Span,
    right_span: Span,
    condition: Box<dyn AstNode>,
    if_true: Box<dyn AstNode>,
    if_false: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl Ternary {
    pub fn new(
        left_span: Span,
        right_span: Span,
        condition: Box<dyn AstNode>,
        if_true: Box<dyn AstNode>,
        if_false: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> Ternary {
        Ternary {
            left_span,
            right_span,
            condition,
            if_true,
            if_false,
            next,
        }
    }

    fn print_label_ternary(&self, own_address: *const c_void) {
        println!("{:p} [label=\"?:\"];", own_address);
    }
}

impl AstNode for Ternary {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.condition.as_ref(), own_address);
        print_dependencies_own(self.if_true.as_ref(), own_address);
        print_dependencies_own(self.if_false.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.condition.as_ref(), own_address);
        print_dependencies_child(self.if_true.as_ref(), own_address);
        print_dependencies_child(self.if_false.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        self.print_label_ternary(own_address);
        print_labels_child(self.condition.as_ref(), lexer);
        print_labels_child(self.if_true.as_ref(), lexer);
        print_labels_child(self.if_false.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        _stack: &mut ScopeStack,
        _lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        //TO DO: Implement a boolean return here.
        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.left_span
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct Binary {
    node_id: Span,
    op_type: BinaryType,
    lhs: Box<dyn AstNode>,
    rhs: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl Binary {
    pub fn new(
        node_id: Span,
        op_type: BinaryType,
        lhs: Box<dyn AstNode>,
        rhs: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> Binary {
        Binary {
            node_id,
            op_type,
            lhs,
            rhs,
            next,
        }
    }

    fn binary_evaluation(
        &self,
        left_value: SymbolType,
        right_value: SymbolType,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<SymbolType, CompilerError> {
        match &self.op_type {
            BinaryType::BoolOr => match (
                left_value.to_bool(self.node_id, lexer)?,
                right_value.to_bool(self.node_id, lexer)?,
            ) {
                (None, _) | (_, None) => Ok(SymbolType::Bool(None)),
                (Some(left_value), Some(right_value)) => {
                    Ok(SymbolType::Bool(Some(left_value || right_value)))
                }
            },
            BinaryType::BoolAnd => match (
                left_value.to_bool(self.node_id, lexer)?,
                right_value.to_bool(self.node_id, lexer)?,
            ) {
                (None, _) | (_, None) => Ok(SymbolType::Bool(None)),
                (Some(left_value), Some(right_value)) => {
                    Ok(SymbolType::Bool(Some(left_value && right_value)))
                }
            },
            BinaryType::BitOr => match (
                left_value.to_int(self.node_id, lexer)?,
                right_value.to_int(self.node_id, lexer)?,
            ) {
                (None, _) | (_, None) => Ok(SymbolType::Bool(None)),
                (Some(left_value), Some(right_value)) => {
                    Ok(SymbolType::Int(Some(left_value | right_value)))
                }
            },
            BinaryType::BitXor => match (
                left_value.to_int(self.node_id, lexer)?,
                right_value.to_int(self.node_id, lexer)?,
            ) {
                (None, _) | (_, None) => Ok(SymbolType::Bool(None)),
                (Some(left_value), Some(right_value)) => {
                    Ok(SymbolType::Int(Some(left_value ^ right_value)))
                }
            },
            BinaryType::BitAnd => match (
                left_value.to_int(self.node_id, lexer)?,
                right_value.to_int(self.node_id, lexer)?,
            ) {
                (None, _) | (_, None) => Ok(SymbolType::Bool(None)),
                (Some(left_value), Some(right_value)) => {
                    Ok(SymbolType::Int(Some(left_value & right_value)))
                }
            },
            BinaryType::Add => {
                match left_value.associate_with(&right_value, self.node_id, lexer)? {
                    SymbolType::String(_) => match (left_value, right_value) {
                        (SymbolType::String(left_maybe), SymbolType::String(right_maybe)) => {
                            match (left_maybe, right_maybe) {
                                (Some(left_value), Some(right_value)) => Ok(SymbolType::String(
                                    Some(format!("{}{}", left_value, right_value)),
                                )),
                                (Some(left_value), None) => {
                                    Ok(SymbolType::String(Some(format!("{}", left_value))))
                                }
                                (None, Some(right_value)) => {
                                    Ok(SymbolType::String(Some(format!("{}", right_value))))
                                }
                                (None, None) => Ok(SymbolType::String(None)),
                            }
                        }
                        (_, _) => Err(CompilerError::SanityError(
                            "binary_evaluation() on BinaryType::Add found non (String, String)"
                                .to_string(),
                        )),
                    },
                    SymbolType::Char(_) => {
                        let invalid_type = "int or float".to_string();
                        let ((line, col), (_, _)) = lexer.line_col(self.node_id);
                        let highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                        Err(CompilerError::SemanticErrorCharToX {
                            invalid_type,
                            line,
                            col,
                            highlight,
                        })
                    }
                    SymbolType::Bool(_) | SymbolType::Int(_) => {
                        match (
                            left_value.to_int(self.node_id, lexer)?,
                            right_value.to_int(self.node_id, lexer)?,
                        ) {
                            (Some(left_value), Some(right_value)) => {
                                Ok(SymbolType::Int(Some(left_value + right_value)))
                            }
                            (_, _) => Ok(SymbolType::Int(None)),
                        }
                    }
                    SymbolType::Float(_) => match (
                        left_value.to_float(self.node_id, lexer)?,
                        right_value.to_float(self.node_id, lexer)?,
                    ) {
                        (Some(left_value), Some(right_value)) => {
                            Ok(SymbolType::Float(Some(left_value + right_value)))
                        }
                        (_, _) => Ok(SymbolType::Float(None)),
                    },
                }
            }
            BinaryType::Sub => Ok(SymbolType::Bool(Some(false))), // DUMMMY
            BinaryType::Mult => Ok(SymbolType::Bool(Some(false))), // DUMMMY
            BinaryType::Div => Ok(SymbolType::Bool(Some(false))), // DUMMMY
            BinaryType::Mod => Ok(SymbolType::Bool(Some(false))), // DUMMMY
            BinaryType::Equal => Ok(SymbolType::Bool(Some(false))), // DUMMMY
            BinaryType::NotEqual => Ok(SymbolType::Bool(Some(false))), // DUMMMY
            BinaryType::Lesser => Ok(SymbolType::Bool(Some(false))), // DUMMMY
            BinaryType::Greater => Ok(SymbolType::Bool(Some(false))), // DUMMMY
            BinaryType::LesserEqual => Ok(SymbolType::Bool(Some(false))), // DUMMMY
            BinaryType::GreaterEqual => Ok(SymbolType::Bool(Some(false))), // DUMMMY
        }
    }
}

impl AstNode for Binary {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.lhs.as_ref(), own_address);
        print_dependencies_own(self.rhs.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.lhs.as_ref(), own_address);
        print_dependencies_child(self.rhs.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.lhs.as_ref(), lexer);
        print_labels_child(self.rhs.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let left_value_type = match self.lhs.evaluate_node(stack, lexer)? {
            Some(value) => value,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "lhs.evaluate_node() returned None for Binary of type {:?}",
                    self.op_type
                )))
            }
        };
        let right_value_type = match self.rhs.evaluate_node(stack, lexer)? {
            Some(value) => value,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "rhs.evaluate_node() returned None for Binary of type {:?}",
                    self.op_type
                )))
            }
        };

        if let Some(node) = &self.next {
            return Err(CompilerError::SanityError(format!(
                "Binary {:?} has a self.next node: {:?}",
                self.op_type, node
            )));
        };

        Ok(Some(self.binary_evaluation(
            left_value_type,
            right_value_type,
            lexer,
        )?))
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub enum BinaryType {
    BoolOr,
    BoolAnd,
    BitOr,
    BitXor,
    BitAnd,
    Equal,
    NotEqual,
    Lesser,
    Greater,
    LesserEqual,
    GreaterEqual,
    Add,
    Sub,
    Mult,
    Div,
    Mod,
}

#[derive(Debug)]
pub struct Unary {
    node_id: Span,
    op_type: UnaryType,
    operand: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl Unary {
    pub fn new(
        node_id: Span,
        op_type: UnaryType,
        operand: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> Unary {
        Unary {
            node_id,
            op_type,
            operand,
            next,
        }
    }
    fn unary_evaluation(
        &self,
        type_value: SymbolType,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<SymbolType, CompilerError> {
        match &self.op_type {
            UnaryType::Positive => match type_value {
                symbol @ SymbolType::Int(_) | symbol @ SymbolType::Float(_) => Ok(symbol),
                SymbolType::Bool(maybe_value) => match &maybe_value {
                    Some(value) => match value {
                        true => Ok(SymbolType::Int(Some(1))),
                        false => Ok(SymbolType::Int(Some(0))),
                    },
                    None => Ok(SymbolType::Int(None)),
                },
                SymbolType::Char(_) => {
                    let invalid_type = "int or float".to_string();
                    let ((line, col), (_, _)) = lexer.line_col(self.node_id);
                    let highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                    Err(CompilerError::SemanticErrorCharToX {
                        invalid_type,
                        line,
                        col,
                        highlight,
                    })
                }
                SymbolType::String(_) => {
                    let invalid_type = "int or float".to_string();
                    let ((line, col), (_, _)) = lexer.line_col(self.node_id);
                    let highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                    Err(CompilerError::SemanticErrorStringToX {
                        invalid_type,
                        line,
                        col,
                        highlight,
                    })
                }
            },
            UnaryType::Negative => match type_value {
                symbol @ SymbolType::Int(_) | symbol @ SymbolType::Float(_) => Ok(symbol),
                SymbolType::Bool(maybe_value) => match &maybe_value {
                    Some(value) => match value {
                        true => Ok(SymbolType::Int(Some(-1))),
                        false => Ok(SymbolType::Int(Some(0))),
                    },
                    None => Ok(SymbolType::Int(None)),
                },
                SymbolType::Char(_) => {
                    let invalid_type = "int or float".to_string();
                    let ((line, col), (_, _)) = lexer.line_col(self.node_id);
                    let highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                    Err(CompilerError::SemanticErrorCharToX {
                        invalid_type,
                        line,
                        col,
                        highlight,
                    })
                }
                SymbolType::String(_) => {
                    let invalid_type = "int or float".to_string();
                    let ((line, col), (_, _)) = lexer.line_col(self.node_id);
                    let highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                    Err(CompilerError::SemanticErrorStringToX {
                        invalid_type,
                        line,
                        col,
                        highlight,
                    })
                }
            },
            UnaryType::Not => match type_value {
                SymbolType::Int(maybe_value) => match &maybe_value {
                    Some(value) => {
                        if *value == 0i32 {
                            Ok(SymbolType::Bool(Some(true)))
                        } else {
                            Ok(SymbolType::Bool(Some(false)))
                        }
                    }
                    None => Ok(SymbolType::Bool(None)),
                },
                SymbolType::Float(maybe_value) => match &maybe_value {
                    Some(value) => {
                        if *value == 0f64 {
                            Ok(SymbolType::Bool(Some(true)))
                        } else {
                            Ok(SymbolType::Bool(Some(false)))
                        }
                    }
                    None => Ok(SymbolType::Bool(None)),
                },
                SymbolType::Bool(maybe_value) => match &maybe_value {
                    Some(value) => match value {
                        true => Ok(SymbolType::Bool(Some(false))),
                        false => Ok(SymbolType::Bool(Some(true))),
                    },
                    None => Ok(SymbolType::Bool(None)),
                },
                SymbolType::Char(_) => {
                    let invalid_type = "bool".to_string();
                    let ((line, col), (_, _)) = lexer.line_col(self.node_id);
                    let highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                    Err(CompilerError::SemanticErrorCharToX {
                        invalid_type,
                        line,
                        col,
                        highlight,
                    })
                }
                SymbolType::String(_) => {
                    let invalid_type = "bool".to_string();
                    let ((line, col), (_, _)) = lexer.line_col(self.node_id);
                    let highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                    Err(CompilerError::SemanticErrorStringToX {
                        invalid_type,
                        line,
                        col,
                        highlight,
                    })
                }
            },
            UnaryType::Boolean => match type_value {
                SymbolType::Int(maybe_value) => match &maybe_value {
                    Some(value) => {
                        if *value == 0i32 {
                            Ok(SymbolType::Bool(Some(false)))
                        } else {
                            Ok(SymbolType::Bool(Some(true)))
                        }
                    }
                    None => Ok(SymbolType::Bool(None)),
                },
                SymbolType::Float(maybe_value) => match &maybe_value {
                    Some(value) => {
                        if *value == 0f64 {
                            Ok(SymbolType::Bool(Some(false)))
                        } else {
                            Ok(SymbolType::Bool(Some(true)))
                        }
                    }
                    None => Ok(SymbolType::Bool(None)),
                },
                symbol @ SymbolType::Bool(_) => Ok(symbol),
                SymbolType::Char(_) => {
                    let invalid_type = "bool".to_string();
                    let ((line, col), (_, _)) = lexer.line_col(self.node_id);
                    let highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                    Err(CompilerError::SemanticErrorCharToX {
                        invalid_type,
                        line,
                        col,
                        highlight,
                    })
                }
                SymbolType::String(_) => {
                    let invalid_type = "bool".to_string();
                    let ((line, col), (_, _)) = lexer.line_col(self.node_id);
                    let highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                    Err(CompilerError::SemanticErrorStringToX {
                        invalid_type,
                        line,
                        col,
                        highlight,
                    })
                }
            },
            UnaryType::Hash => Ok(SymbolType::Int(None)),
            UnaryType::Address => Ok(type_value),
            UnaryType::Pointer => Ok(type_value),
        }
    }
}

impl AstNode for Unary {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.operand.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.operand.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        print_labels_child(self.operand.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        if let Some(node) = &self.next {
            return Err(CompilerError::LexicalError(format!(
                "Unary struct holds a reference to a next node: {:?}",
                node
            )));
        };

        let type_value = match self.operand.evaluate_node(stack, lexer)? {
            Some(value) => value,
            None => {
                return Err(CompilerError::SemanticError(format!(
                    "Unary struct operand returned no value on evaluation: {:?}",
                    self.operand
                )))
            }
        };

        let type_value = self.unary_evaluation(type_value.clone(), lexer)?;

        if let Some(node) = &self.next {
            return Err(CompilerError::SanityError(format!(
                "Unary {:?} has a self.next node: {:?}",
                self.op_type, node
            )));
        };

        Ok(Some(type_value))
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub enum UnaryType {
    Positive,
    Negative,
    Not,
    Boolean,
    Hash,
    Address,
    Pointer,
}

#[derive(Debug)]
pub struct VarAccess {
    node_id: Span,
    next: Option<Box<dyn AstNode>>,
}

impl VarAccess {
    pub fn new(node_id: Span, next: Option<Box<dyn AstNode>>) -> VarAccess {
        VarAccess { node_id, next }
    }
}

impl AstNode for VarAccess {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        let class = SymbolClass::Var;
        let previous_def = stack.get_previous_def(span, lexer, class)?;
        let type_value = previous_def.type_value.clone();

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(Some(type_value))
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VecAccess {
    node_id: Span,
    vec_name: Box<dyn AstNode>,
    vec_index: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecAccess {
    pub fn new(
        node_id: Span,
        vec_name: Box<dyn AstNode>,
        vec_index: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VecAccess {
        VecAccess {
            node_id,
            vec_name,
            vec_index,
            next,
        }
    }

    fn print_label_vec_access(&self, own_address: *const c_void) {
        println!("{:p} [label=\"[]\"];", own_address);
    }
}

impl AstNode for VecAccess {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(self.vec_name.as_ref(), own_address);
        print_dependencies_own(self.vec_index.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        print_dependencies_child(self.vec_name.as_ref(), own_address);
        print_dependencies_child(self.vec_index.as_ref(), own_address);
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        self.print_label_vec_access(own_address);
        print_labels_child(self.vec_name.as_ref(), lexer);
        print_labels_child(self.vec_index.as_ref(), lexer);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.vec_name.evaluate_node(stack, lexer)?;

        self.vec_index.evaluate_node(stack, lexer)?;

        // TO DO: Add symbol and check type.

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        // TO DO: Implement a return value here.
        Ok(None)
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VarInvoke {
    node_id: Span,
    next: Option<Box<dyn AstNode>>,
}

impl VarInvoke {
    pub fn new(node_id: Span, next: Option<Box<dyn AstNode>>) -> VarInvoke {
        VarInvoke { node_id, next }
    }
}

impl AstNode for VarInvoke {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        let class = SymbolClass::Var;
        let previous_def = stack.get_previous_def(span, lexer, class)?;
        let type_value = previous_def.type_value.clone();

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(Some(type_value))
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VecInvoke {
    node_id: Span,
    next: Option<Box<dyn AstNode>>,
}

impl VecInvoke {
    pub fn new(node_id: Span, next: Option<Box<dyn AstNode>>) -> VecInvoke {
        VecInvoke { node_id, next }
    }
}

impl AstNode for VecInvoke {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        let class = SymbolClass::Vec;
        let previous_def = stack.get_previous_def(span, lexer, class)?;
        let type_value = previous_def.type_value.clone();

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(Some(type_value))
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct LiteralInt {
    node_id: Span,
    next: Option<Box<dyn AstNode>>,
}

impl LiteralInt {
    pub fn new(node_id: Span, next: Option<Box<dyn AstNode>>) -> LiteralInt {
        LiteralInt { node_id, next }
    }
}

impl AstNode for LiteralInt {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let class = SymbolClass::Lit;

        let span = self.node_id;
        let id = lexer.span_str(span).to_string();

        let var_value = match id.parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                return Err(CompilerError::LexicalError(format!(
                    "Unable to parse {} into i32.",
                    id
                )))
            }
        };
        let var_type = SymbolType::Int(Some(var_value));

        let ((line, col), (_, _)) = lexer.line_col(span);
        let our_symbol = CallSymbol::new(id, span, line, col, var_type.clone(), class);

        stack.push_symbol(our_symbol)?;

        if let Some(node) = &self.next {
            node.evaluate_node(stack, lexer)?;
        };

        Ok(Some(var_type))
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct LiteralFloat {
    node_id: Span,
    next: Option<Box<dyn AstNode>>,
}

impl LiteralFloat {
    pub fn new(node_id: Span, next: Option<Box<dyn AstNode>>) -> LiteralFloat {
        LiteralFloat { node_id, next }
    }
}

impl AstNode for LiteralFloat {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let class = SymbolClass::Lit;

        let span = self.node_id;
        let id = lexer.span_str(span).to_string();

        let var_value = match id.parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                return Err(CompilerError::LexicalError(format!(
                    "Unable to parse {} into f64.",
                    id
                )))
            }
        };
        let var_type = SymbolType::Float(Some(var_value));

        let ((line, col), (_, _)) = lexer.line_col(span);
        let our_symbol = CallSymbol::new(id, span, line, col, var_type.clone(), class);

        stack.push_symbol(our_symbol)?;

        Ok(Some(var_type))
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct LiteralBool {
    node_id: Span,
    next: Option<Box<dyn AstNode>>,
}

impl LiteralBool {
    pub fn new(node_id: Span, next: Option<Box<dyn AstNode>>) -> LiteralBool {
        LiteralBool { node_id, next }
    }
}

impl AstNode for LiteralBool {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let class = SymbolClass::Lit;

        let span = self.node_id;
        let id = lexer.span_str(span).to_string();

        let var_value = match id.parse::<bool>() {
            Ok(value) => value,
            Err(_) => {
                return Err(CompilerError::LexicalError(format!(
                    "Unable to parse {} into bool.",
                    id
                )))
            }
        };
        let var_type = SymbolType::Bool(Some(var_value));

        let ((line, col), (_, _)) = lexer.line_col(span);
        let our_symbol = CallSymbol::new(id, span, line, col, var_type.clone(), class);

        stack.push_symbol(our_symbol)?;

        Ok(Some(var_type))
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct LiteralChar {
    node_id: Span,
    next: Option<Box<dyn AstNode>>,
}

impl LiteralChar {
    pub fn new(node_id: Span, next: Option<Box<dyn AstNode>>) -> LiteralChar {
        LiteralChar { node_id, next }
    }

    fn print_label_lit_char(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        let text = lexer.span_str(self.node_id);
        println!(
            "{:p} [label=\"{}\"];",
            own_address,
            &text[1..(text.len() - 1)]
        );
    }
}

impl AstNode for LiteralChar {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        self.print_label_lit_char(lexer, own_address);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let class = SymbolClass::Lit;

        let span = self.node_id;
        let id = lexer.span_str(span).to_string();

        let id_chars = id.as_bytes();
        if id_chars.len() != 3 {
            return Err(CompilerError::SanityError(
                "on evaluate_node(), character not 3 characters long ".to_string(),
            ));
        }

        let var_value = id_chars[1];
        let var_type = SymbolType::Char(Some(var_value));

        let ((line, col), (_, _)) = lexer.line_col(span);
        let our_symbol = CallSymbol::new(id, span, line, col, var_type.clone(), class);

        stack.push_symbol(our_symbol)?;

        Ok(Some(var_type))
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct LiteralString {
    node_id: Span,
    next: Option<Box<dyn AstNode>>,
}

impl LiteralString {
    pub fn new(node_id: Span, next: Option<Box<dyn AstNode>>) -> LiteralString {
        LiteralString { node_id, next }
    }

    fn print_label_lit_string(
        &self,
        lexer: &dyn NonStreamingLexer<u32>,
        own_address: *const c_void,
    ) {
        let text = lexer.span_str(self.node_id);
        println!(
            "{:p} [label=\"{}\"];",
            own_address,
            &text[1..(text.len() - 1)]
        );
    }
}

impl AstNode for LiteralString {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        self.print_label_lit_string(lexer, own_address);
        if let Some(next_node) = &self.next {
            print_labels_next(next_node.as_ref(), own_address, lexer)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let class = SymbolClass::Lit;

        let span = self.node_id;
        let id = lexer.span_str(span).to_string();

        if id.len() < 2 {
            return Err(CompilerError::SanityError(
                "on evaluate_node(), string smaller than 2 characters (no \") ".to_string(),
            ));
        }
        let clean_string = (&id[1..id.len() - 1]).to_string();

        let var_type = SymbolType::String(Some(clean_string));

        let ((line, col), (_, _)) = lexer.line_col(span);
        let our_symbol = CallSymbol::new(id, span, line, col, var_type.clone(), class);

        stack.push_symbol(our_symbol)?;

        Ok(Some(var_type))
    }
    fn get_id(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

fn print_dependencies_ripple(next_node: &(dyn AstNode), own_address: *const c_void, ripple: bool) {
    if next_node.is_tree_member() {
        let next_address = addr_of!(*next_node) as *const c_void;
        if ripple {
            println!("{:p}, {:p}", own_address, next_address);
        }
        next_node.print_dependencies(next_address, false);
    } else {
        next_node.print_dependencies(own_address, ripple);
    }
}

fn print_dependencies_own(child: &(dyn AstNode), own_address: *const c_void) {
    if child.is_tree_member() {
        let child_address = addr_of!(*child) as *const c_void;
        println!("{:p}, {:p}", own_address, child_address);
    }
}

fn print_dependencies_child(child: &(dyn AstNode), own_address: *const c_void) {
    if child.is_tree_member() {
        let child_address = addr_of!(*child) as *const c_void;
        child.print_dependencies(child_address, false);
    } else {
        child.print_dependencies(own_address, true);
    }
}

fn print_dependencies_own_next(next_node: &(dyn AstNode), own_address: *const c_void) {
    if next_node.is_tree_member() {
        let next_address = addr_of!(*next_node) as *const c_void;
        println!("{:p}, {:p}", own_address, next_address);
    }
}

fn print_dependencies_next(next_node: &(dyn AstNode), own_address: *const c_void) {
    if next_node.is_tree_member() {
        let next_address = addr_of!(*next_node) as *const c_void;
        next_node.print_dependencies(next_address, false);
    } else {
        next_node.print_dependencies(own_address, true);
    }
}

fn print_label_self(
    self_span: Span,
    lexer: &dyn NonStreamingLexer<u32>,
    own_address: *const c_void,
) {
    println!(
        "{:p} [label=\"{}\"];",
        own_address,
        lexer.span_str(self_span)
    );
}

fn print_labels_child(child: &(dyn AstNode), lexer: &dyn NonStreamingLexer<u32>) {
    child.print_labels(lexer, addr_of!(*child) as *const c_void);
}

fn print_labels_next(
    next_node: &(dyn AstNode),
    own_address: *const c_void,
    lexer: &dyn NonStreamingLexer<u32>,
) {
    if next_node.is_tree_member() {
        let next_address = addr_of!(*next_node) as *const c_void;
        next_node.print_labels(lexer, next_address);
    } else {
        next_node.print_labels(lexer, own_address);
    }
}

fn append_node(
    current_next: &mut Option<Box<dyn AstNode>>,
    new_last: Box<dyn AstNode>,
) -> Option<Box<dyn AstNode>> {
    match current_next.take() {
        Some(mut node) => {
            node.append_to_next(new_last);
            Some(node)
        }
        None => Some(new_last),
    }
}
