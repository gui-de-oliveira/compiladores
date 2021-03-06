// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

use lrpar::{NonStreamingLexer, Span};
use std::ffi::c_void;
use std::ptr::addr_of;

use super::ast_node::AstNode;
use super::error::CompilerError;
use super::instructions::{CodeLine, IlocCode, Instruction, Operation, Register};
use super::semantic_structures::{ BoolValue,
    CallSymbol, DefSymbol, IntValue, ScopeStack, SymbolClass, SymbolType, INT_SIZE,
};

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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        stack.check_duplicate(span, lexer)?;

        let id = lexer.span_str(self.node_id).to_string();
        let var_type = SymbolType::from_str(lexer.span_str(self.var_type))?;
        let offset = stack.get_offset()?;
        let var_type = match var_type {
            SymbolType::Int(IntValue::Undefined) => {
                SymbolType::Int(IntValue::Memory(Register::Rbss, offset))
            }
            _ => {
                return Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                    "Global variable of unsuported type declared: {} ({})",
                    id,
                    lexer.span_str(self.var_type)
                )))
            }
        };
        let ((line, col), (_, _)) = lexer.line_col(self.node_id);
        let is_global = true;
        let class = SymbolClass::Var { is_global, offset };
        let size = var_type.get_symbol_type_size();

        let our_symbol = DefSymbol::new(
            id,
            span,
            line,
            col,
            var_type,
            class,
            Some(size),
            Register::Rbss,
            offset,
        );

        stack.add_def_symbol(our_symbol)?;
        stack.add_offset(size)?;

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        stack.check_duplicate(span, lexer)?;
        let id = lexer.span_str(self.node_id).to_string();
        let offset = stack.get_offset()?;

        let var_type = match SymbolType::from_str(lexer.span_str(self.var_type))? {
            SymbolType::String(_) => {
                let start = self.var_type.start();
                let end = self.vec_size.get_span().end() + 1;
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
            SymbolType::Int(IntValue::Undefined) => {
                SymbolType::Int(IntValue::Memory(Register::Rbss, offset))
            }
            _ => {
                return Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                    "Global variable of unsuported type declared: {} ({})",
                    id,
                    lexer.span_str(self.var_type)
                )))
            }
        };
        let ((line, col), (_, _)) = lexer.line_col(self.node_id);
        let class = SymbolClass::Vec { offset };

        let vec_size = self.vec_size.evaluate_node(code, stack, lexer)?;

        let size_int = match vec_size {
            Some(SymbolType::Int(IntValue::Literal(size_int))) => size_int as u32,
            _ => {
                return Err(CompilerError::SanityError(format!(
                    "vec_size symbol invalid (on GlobalVecDef.evaluate_node()): {:?}",
                    vec_size
                )))
            }
        };

        let base_size = var_type.get_symbol_type_size();
        let size = base_size * size_int;

        for i in 0..size {
            let index_id = format!("{}[{}]", &id, i);
            let index_symbol = DefSymbol::new(
                index_id,
                span,
                line,
                col,
                var_type.clone(),
                class.clone(),
                Some(base_size),
                Register::Rbss,
                offset + (i * base_size),
            );
            stack.add_def_symbol(index_symbol)?;
        }

        let our_symbol = DefSymbol::new(
            id,
            span,
            line,
            col,
            var_type,
            class,
            Some(size),
            Register::Rbss,
            offset,
        );
        stack.add_def_symbol(our_symbol)?;
        stack.add_offset(size)?;

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
    first_command: Option<Box<dyn AstNode>>,
    next: Option<Box<dyn AstNode>>,
}

const FN_OFFSET: u32 = 16;

impl FnDef {
    pub fn new(
        is_static: bool,
        return_type: Span,
        node_id: Span,
        params: Vec<Parameter>,
        first_command: Option<Box<dyn AstNode>>,
        next: Option<Box<dyn AstNode>>,
    ) -> FnDef {
        FnDef {
            is_static,
            return_type,
            node_id,
            params,
            first_command,
            next,
        }
    }
}

impl AstNode for FnDef {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        if let Some(command) = &self.first_command {
            print_dependencies_own(command.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        if let Some(command) = &self.first_command {
            print_dependencies_child(command.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.node_id, lexer, own_address);
        if let Some(command) = &self.first_command {
            print_labels_child(command.as_ref(), lexer)
        }
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        stack.check_duplicate(span, lexer)?;
        let offset = stack.get_offset()?;

        let return_type = SymbolType::from_str(lexer.span_str(self.return_type))?;
        let id = lexer.span_str(self.node_id).to_string();
        let ((line, col), (_, _)) = lexer.line_col(self.node_id);
        let class = SymbolClass::Fn(self.params.clone());
        let our_symbol = DefSymbol::new(
            id.clone(),
            span,
            line,
            col,
            return_type.clone(),
            class,
            None,
            Register::Rbss,
            offset,
        );

        stack.add_scope(Some(return_type));
        stack.add_offset(FN_OFFSET)?;
        let mut starting_size = FN_OFFSET;
        for param in self.params.iter() {
            starting_size += param.evaluate_param(code, stack, lexer)?;
        }

        let new_label = code.add_fn_label(id);
        code.push_code(CodeLine::Deliver(Instruction::Labeled(new_label, Operation::Nop)));
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::I2i(
            Register::Rsp,
            Register::Rfp,
        ))));
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::AddI(
            Register::Rsp,
            starting_size as i32,
            Register::Rsp,
        ))));

        if let Some(command) = &self.first_command {
            command.evaluate_node(code, stack, lexer)?;
        };

        stack.remove_scope()?;

        stack.add_def_symbol(our_symbol)?;

        let return_addr_reg = code.new_register();
        let restore_rsp_reg = code.new_register();
        let restore_rfp_reg = code.new_register();
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
            Register::Rfp,
            0,
            return_addr_reg,
        ))));
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
            Register::Rfp,
            4,
            restore_rsp_reg,
        ))));
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
            Register::Rfp,
            8,
            restore_rfp_reg,
        ))));
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::I2i(
            restore_rsp_reg,
            Register::Rsp,
        ))));
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::I2i(
            restore_rfp_reg,
            Register::Rfp,
        ))));
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Jump(return_addr_reg))));

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Parameter {
    pub is_const: bool,
    pub param_type: Span,
    pub node_id: Span,
}

impl Parameter {
    fn evaluate_param(
        &self,
        _code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<u32, CompilerError> {
        let span = self.node_id;

        stack.check_duplicate(span, lexer)?;

        let var_type = SymbolType::from_str(lexer.span_str(self.param_type))?;

        if let SymbolType::String(_) = var_type {
            let ((line, col), (_, _)) = lexer.line_col(span);
            let highlight = ScopeStack::form_string_highlight(span, lexer);
            return Err(CompilerError::SemanticErrorFunctionString {
                id: lexer.span_str(span).to_string(),
                line,
                col,
                highlight,
            });
        };

        let id = lexer.span_str(self.node_id).to_string();
        let ((line, col), (_, _)) = lexer.line_col(self.node_id);
        let is_global = false;
        let offset = stack.get_offset()?;
        let class = SymbolClass::Var { is_global, offset };
        let size = var_type.get_symbol_type_size();
        let our_symbol = DefSymbol::new(
            id,
            span,
            line,
            col,
            var_type,
            class,
            Some(size),
            Register::Rfp,
            offset,
        );

        stack.add_def_symbol(our_symbol)?;
        stack.add_offset(size)?;

        Ok(size)
    }

    fn get_symbol_type(
        &self,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<SymbolType, CompilerError> {
        SymbolType::from_str(lexer.span_str(self.param_type))
    }
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        stack.check_duplicate(span, lexer)?;

        let id = lexer.span_str(self.node_id).to_string();
        let var_type = SymbolType::from_str(lexer.span_str(self.var_type))?;
        let offset = stack.get_offset()?;
        let var_type = match var_type {
            SymbolType::Int(IntValue::Undefined) => {
                SymbolType::Int(IntValue::Memory(Register::Rfp, offset))
            }
            _ => {
                return Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                    "Local variable of unsuported type declared: {} ({})",
                    id,
                    lexer.span_str(self.var_type)
                )))
            }
        };
        let ((line, col), (_, _)) = lexer.line_col(self.node_id);
        let is_global = false;
        let class = SymbolClass::Var { is_global, offset };
        let size = var_type.get_symbol_type_size();

        let our_symbol = DefSymbol::new(
            id,
            span,
            line,
            col,
            var_type,
            class,
            Some(size),
            Register::Rfp,
            offset,
        );

        stack.add_def_symbol(our_symbol)?;
        stack.add_offset(size)?;

        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::AddI(
            Register::Rsp,
            size as i32,
            Register::Rsp,
        ))));

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.var_def.evaluate_node(code, stack, lexer)?;
        self.var_value.evaluate_node(code, stack, lexer)?;

        let def_symbol =
            stack.get_previous_def(self.var_def.get_span(), lexer, SymbolClass::default_var())?;
        let var_symbol =
            stack.get_previous_def(self.var_value.get_span(), lexer, SymbolClass::default_var())?;

        let id_symbol_type = &var_symbol.type_value;
        let _updated_symbol = def_symbol.cast_or_scream(id_symbol_type, self.node_id, lexer, false)?;

        if let SymbolType::Int(IntValue::Memory(offset_source, offset)) = id_symbol_type {
            let new_register = code.new_register();
            code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                *offset_source,
                *offset as i32,
                new_register,
            ))));
            code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
                new_register,
                def_symbol.offset_source,
                def_symbol.offset as i32,
            ))));
        };

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VarDefInitLit {
    node_id: Span,
    var_def: Box<LocalVarDef>,
    var_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VarDefInitLit {
    pub fn new(
        node_id: Span,
        var_def: Box<LocalVarDef>,
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.var_def.evaluate_node(code, stack, lexer)?;

        let lit_symbol_type = match self.var_value.evaluate_node(code, stack, lexer)? {
            Some(good @ SymbolType::Int(IntValue::Literal(_))) => good,
            Some(bad) => {
                return Err(CompilerError::SanityError(format!(
                    "VarDefInitLit found bad SymbolType (on self.var_value.evaluate_node()): {:?}",
                    bad
                )))
            }
            None => {
                return Err(CompilerError::SanityError(format!(
                    "VarDefInitLit found no SymbolType (on self.var_value.evaluate_node())"
                )))
            }
        };

        let def_symbol = {
            let span = self.var_def.get_span();
            let def_symbol = stack.get_previous_def(span, lexer, SymbolClass::default_var())?;
            def_symbol.clone()
        };

        let _updated_symbol =
            def_symbol.cast_or_scream(&lit_symbol_type, self.node_id, lexer, false)?; // Legacy from previous version.

        if let SymbolType::Int(IntValue::Literal(num)) = lit_symbol_type {
            let new_register = code.new_register();
            code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                num,
                new_register,
            ))));
            code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
                new_register,
                def_symbol.offset_source,
                def_symbol.offset as i32,
            ))));
        };

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.var_name.evaluate_node(code, stack, lexer)?;
        self.shift_amount.evaluate_node(code, stack, lexer)?;

        let symbol = match stack.pop_symbol()? {
            Some(symbol) => symbol,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "failed to pop expected literal int symbol (on varLeftShift.evaluate_node())"
                )))
            }
        };

        match symbol.type_value {
            SymbolType::Int(IntValue::Literal(value)) => {
                if value > 16 {
                    let shift_amount_span = self.shift_amount.get_span();
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
            _ => {
                return Err(CompilerError::SanityError(format!(
                    "shift_amount received is NOT a literal int (on varLeftShift.evaluate_node())"
                )))
            }
        }

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.var_name.evaluate_node(code, stack, lexer)?;
        self.shift_amount.evaluate_node(code, stack, lexer)?;

        let symbol = match stack.pop_symbol()? {
            Some(symbol) => symbol,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "failed to pop expected literal int symbol (on varRightShift.evaluate_node())"
                )))
            }
        };

        match symbol.type_value {
            SymbolType::Int(IntValue::Literal(value)) => {
                if value > 16 {
                    let shift_amount_span = self.shift_amount.get_span();
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
            _ => {
                return Err(CompilerError::SanityError(format!(
                    "shift_amount received is NOT a literal int (on varRightShift.evaluate_node())"
                )))
            }
        }

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VecLeftShift {
    node_id: Span,
    vec_access: Box<VecAccess>,
    shift_amount: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecLeftShift {
    pub fn new(
        node_id: Span,
        vec_access: Box<VecAccess>,
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.vec_access.evaluate_node(code, stack, lexer)?;

        self.shift_amount.evaluate_node(code, stack, lexer)?;

        let symbol = match stack.pop_symbol()? {
            Some(symbol) => symbol,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "failed to pop expected literal int symbol (on vecLeftShift.evaluate_node())"
                )))
            }
        };

        match symbol.type_value {
            SymbolType::Int(IntValue::Literal(value)) => {
                if value > 16 {
                    let shift_amount_span = self.shift_amount.get_span();
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
            _ => {
                return Err(CompilerError::SanityError(format!(
                    "shift_amount received is NOT a literal int (on vecLeftShift.evaluate_node())"
                )))
            }
        }

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VecRightShift {
    node_id: Span,
    vec_access: Box<VecAccess>,
    shift_amount: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecRightShift {
    pub fn new(
        node_id: Span,
        vec_access: Box<VecAccess>,
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.vec_access.evaluate_node(code, stack, lexer)?;

        self.shift_amount.evaluate_node(code, stack, lexer)?;

        let symbol = match stack.pop_symbol()? {
            Some(symbol) => symbol,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "failed to pop expected literal int symbol (on vecRightShift.evaluate_node())"
                )))
            }
        };

        match symbol.type_value {
            SymbolType::Int(IntValue::Literal(value)) => {
                if value > 16 {
                    let shift_amount_span = self.shift_amount.get_span();
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
            _ => {
                return Err(CompilerError::SanityError(format!(
                    "shift_amount received is NOT a literal int (on vecRightShift.evaluate_node())"
                )))
            }
        }

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VarSet {
    node_id: Span,
    var_name: Box<VarInvoke>,
    new_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VarSet {
    pub fn new(
        node_id: Span,
        var_name: Box<VarInvoke>,
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.var_name.evaluate_node(code, stack, lexer)?;
        let new_value_symbol =
            self.new_value
                .evaluate_node(code, stack, lexer)?
                .ok_or(CompilerError::SanityError(format!(
                    "New value has no SymbolType (on VarSet.evaluate_node())"
                )))?;

        let def_symbol =
            stack.get_previous_def(self.var_name.get_span(), lexer, SymbolClass::default_var())?;

        let _updated_symbol =
            def_symbol.cast_or_scream(&new_value_symbol, self.node_id, lexer, true)?;

        let setter_register;
        match new_value_symbol {
            SymbolType::Int(IntValue::Temp(register)) => {
                setter_register = register;
            },
            SymbolType::Int(IntValue::Literal(number)) => {
                setter_register = code.new_register();
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                    number,
                    setter_register,
                ))));
            },
            SymbolType::Int(IntValue::Memory(register, offset)) => {
                setter_register = code.new_register();
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                    register,
                    offset as i32,
                    setter_register,
                ))));
            },
            SymbolType::Int(IntValue::Undefined) => {
                return Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                    "VarSet called with uninitialized right value.\nLeft value: {:?}\n Right value: {:?}",
                    def_symbol, new_value_symbol
                )))
            }
            _ => {
                return Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                    "VarSet called with unsuported type.\nLeft value: {:?}\n Right value: {:?}",
                    def_symbol, new_value_symbol
                )))
            }
        }
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
            setter_register,
            def_symbol.offset_source,
            def_symbol.offset as i32,
        ))));

        let return_symbol = Some(SymbolType::Int(IntValue::Memory(def_symbol.offset_source, def_symbol.offset)));

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(return_symbol)
    }
    fn get_span(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct VecSet {
    node_id: Span,
    vec_access: Box<VecAccess>,
    new_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecSet {
    pub fn new(
        node_id: Span,
        vec_access: Box<VecAccess>,
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {

        let new_value_symbol =
            self.new_value
                .evaluate_node(code, stack, lexer)?
                .ok_or(CompilerError::SanityError(format!(
                    "New value has no SymbolType (on VecSet.evaluate_node())"
                )))?;

        let def_symbol =
            stack.get_previous_def(self.vec_access.vec_name.get_span(), lexer, SymbolClass::default_vec())?;

        let _updated_symbol =
            def_symbol.cast_or_scream(&new_value_symbol, self.node_id, lexer, true)?;

        let setter_register;
        match new_value_symbol {
            SymbolType::Int(IntValue::Temp(register)) => {
                setter_register = register;
            },
            SymbolType::Int(IntValue::Literal(number)) => {
                setter_register = code.new_register();
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                    number,
                    setter_register,
                ))));
            },
            SymbolType::Int(IntValue::Memory(register, offset)) => {
                setter_register = code.new_register();
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                    register,
                    offset as i32,
                    setter_register,
                ))));
            },
            SymbolType::Int(IntValue::Undefined) => {
                return Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                    "VarSet called with uninitialized right value.\nLeft value: {:?}\n Right value: {:?}",
                    def_symbol, new_value_symbol
                )))
            }
            _ => {
                return Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                    "VarSet called with unsuported type.\nLeft value: {:?}\n Right value: {:?}",
                    def_symbol, new_value_symbol
                )))
            }
        }

        let return_symbol;

        match self.vec_access.evaluate_node(code, stack, lexer)?.ok_or(CompilerError::ParsingErrors(format!("vec_access.evaluate_node() returned no type value for index expression in VecSet.evaluate_node()")))? {
            SymbolType::Int(IntValue::Memory(offset_source, offset)) => {
                return_symbol = Some(SymbolType::Int(IntValue::Memory(offset_source, offset)));
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
                    setter_register,
                    offset_source,
                    offset as i32,
                ))));
            },
            bad => return Err(CompilerError::SanityError(format!(
                "vec_access.evaluate_node() on VecSet returned something different from a memory location: {:?}", bad
            ))),
        }

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(return_symbol)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.var_name.evaluate_node(code, stack, lexer)?;

        let id = self.var_name.get_span();
        let var_def = stack.get_previous_def(id, lexer, SymbolClass::default_var())?;

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
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.var_name.evaluate_node(code, stack, lexer)?;

        let id = self.var_name.get_span();
        let var_def = stack.get_previous_def(id, lexer, SymbolClass::default_var())?;

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
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.lit_value.evaluate_node(code, stack, lexer)?;

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
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

const RETURN_VAL_OFFSET: i32 = 12;

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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let current_scope_type = stack.get_current_scope_type()?;
        let return_value_type = &self.ret_value.evaluate_node(code, stack, lexer)?.ok_or(
            CompilerError::SanityError(format!(
                "Return got no return_value_type from ret_value.evaluate_node(): {:?}",
                self.ret_value
            )),
        )?;

        match return_value_type {
            SymbolType::String(_) => {
                let span = self.ret_value.get_span();
                let id = lexer.span_str(span).to_string();
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                return Err(CompilerError::SemanticErrorFunctionString {
                    id,
                    line,
                    col,
                    highlight,
                });
            }
            SymbolType::Int(return_value) => {
                match return_value {
                    IntValue::Undefined => {
                        return Err(CompilerError::SanityError(format!("Undefined Int as return value in Return function.")))
                    }
                    IntValue::Temp(register) => {
                        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(*register, Register::Rfp, RETURN_VAL_OFFSET))))
                    }
                    IntValue::Memory(offset_source, offset) => {
                        let new_register = code.new_register();
                        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                            *offset_source,
                            *offset as i32,
                            new_register,
                        ))));
                        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(new_register, Register::Rfp, RETURN_VAL_OFFSET))))
                    }
                    IntValue::Literal(number) => {
                        let new_register = code.new_register();
                        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                            *number,
                            new_register,
                        ))));
                        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(new_register, Register::Rfp, RETURN_VAL_OFFSET))))
                    }
                }
                let rsp_reg = code.new_register();
                let rfp_reg = code.new_register();
                let jump_reg = code.new_register();
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                    Register::Rfp,
                    0,
                    rsp_reg,
                ))));
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                    Register::Rfp,
                    4,
                    rfp_reg,
                ))));
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                    Register::Rfp,
                    8,
                    jump_reg,
                ))));
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::I2i(
                    rsp_reg,
                    Register::Rsp,
                ))));
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::I2i(
                    rfp_reg,
                    Register::Rfp,
                ))));
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Jump(
                    jump_reg,
                ))));
            }
            _ => return Err(CompilerError::IlocErrorUndefinedBehavior(format!("Unsuported type usage for Return in function: {}", return_value_type.to_str().to_string()))),
        }

        match current_scope_type.associate_with(return_value_type, self.node_id, lexer) {
            Ok(SymbolType::Int(_)) => (),
            Ok(_) => return Err(CompilerError::IlocErrorUndefinedBehavior(format!("Unsuported type usage for Return in function: {}", return_value_type.to_str().to_string()))),
            Err(_) => {
                let id = self.ret_value.get_span();
                let expected_type = current_scope_type.to_str().to_string();
                let received_type = return_value_type.to_str().to_string();
                let highlight = ScopeStack::form_string_highlight(id, lexer);
                let ((line, col), (_, _)) = lexer.line_col(id);
                return Err(CompilerError::SemanticErrorWrrongParReturn {
                    expected_type,
                    received_type,
                    line,
                    col,
                    highlight,
                });
            }
        }

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct FnCall {
    node_id: Span,
    args: Vec<Box<dyn AstNode>>,
    next: Option<Box<dyn AstNode>>,
}

impl FnCall {
    pub fn new(
        node_id: Span,
        args: Vec<Box<dyn AstNode>>,
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
        for arg in &self.args {
            print_dependencies_own(arg.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_own_next(next_node.as_ref(), own_address);
        }
        for arg in &self.args {
            print_dependencies_child(arg.as_ref(), own_address);
        }
        if let Some(next_node) = &self.next {
            print_dependencies_next(next_node.as_ref(), own_address);
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        self.print_label_fn_call(lexer, own_address);
        for arg in &self.args {
            print_labels_child(arg.as_ref(), lexer);
        }
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        let class = SymbolClass::Fn(vec![]);
        let parameters = {
            let previous_def = stack.get_previous_def(span, lexer, class.clone())?;
            match &previous_def.class {
                SymbolClass::Fn(params) => params.clone(),
                _ => {
                    return Err(CompilerError::SanityError(
                        "FnCall.evaluate_node() received an invalid class from previous def."
                            .to_string(),
                    ))
                }
            }
        };
        let args_num = self.args.len();
        let params_num = parameters.len();

        let id = lexer.span_str(self.node_id).to_string();
        if args_num != params_num {
            let previous_def = stack.get_previous_def(span, lexer, class)?;
            let first_line = previous_def.line;
            let first_col = previous_def.col;
            let first_highlight = ScopeStack::form_string_highlight(previous_def.span, lexer);
            let ((second_line, second_col), (_, _)) = lexer.line_col(self.node_id);
            let second_highlight = ScopeStack::form_string_highlight(self.node_id, lexer);

            return Err(if args_num < params_num {
                CompilerError::SemanticErrorMissingArgs {
                    id,
                    first_line,
                    first_col,
                    first_highlight,
                    second_line,
                    second_col,
                    second_highlight,
                }
            } else {
                CompilerError::SemanticErrorExcessArgs {
                    id,
                    first_line,
                    first_col,
                    first_highlight,
                    second_line,
                    second_col,
                    second_highlight,
                }
            });
        }

        let mut next_param_loc = FN_OFFSET as i32;
        if params_num > 0 {
            let mut param_types = vec![];
            for param in &parameters {
                let param_type = param.get_symbol_type(lexer)?;
                param_types.push(param_type);
            }
            for (i, arg) in self.args.iter().enumerate() {
                let arg_type =
                    arg.evaluate_node(code, stack, lexer)?
                        .ok_or(CompilerError::SanityError(format!(
                            "FnCall error; .evaluate_node() on arg returned no type: {:?}",
                            arg
                        )))?;
                match (arg_type, &param_types[i]) {
                    (SymbolType::String(_), _) | (_, SymbolType::String(_)) => {
                        let param = parameters[i];
                        let span = param.node_id;
                        let id = lexer.span_str(span).to_string();
                        let ((line, col), (_, _)) = lexer.line_col(span);
                        let highlight = ScopeStack::form_string_highlight(span, lexer);
                        return Err(CompilerError::SemanticErrorFunctionString {
                            id,
                            line,
                            col,
                            highlight,
                        });
                    }
                    (SymbolType::Char(_), SymbolType::Char(_)) => continue,
                    (SymbolType::Char(_), _) | (_, SymbolType::Char(_)) => (),
                    (SymbolType::Int(int_value), SymbolType::Int(_)) => {
                        match int_value {
                            bad @ IntValue::Undefined => return Err(CompilerError::IlocErrorUndefinedBehavior(format!("Usage of undefined int type as function parameter for \"{}()\":\n{:?}", id, bad))),
                            IntValue::Temp(register) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
                                    register,
                                    Register::Rsp,
                                    next_param_loc,
                                ))));
                            }
                            IntValue::Memory(offset_source, offset) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    offset_source,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
                                    new_register,
                                    Register::Rsp,
                                    next_param_loc,
                                ))));
                            }
                            IntValue::Literal(number) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    number,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
                                    new_register,
                                    Register::Rsp,
                                    next_param_loc,
                                ))));
                            }
                        };
                        next_param_loc += INT_SIZE as i32;
                        continue;
                    },
                    (bad, worse) => return Err(CompilerError::IlocErrorUndefinedBehavior(format!("Usage of unsuported types as function parameters for \"{}()\":\n{:?}\n{:?}", id, bad, worse))),
                }
                let id = lexer.span_str(self.node_id).to_string();
                let previous_def = stack.get_previous_def(span, lexer, class)?;
                let first_line = previous_def.line;
                let first_col = previous_def.col;
                let first_highlight = ScopeStack::form_string_highlight(previous_def.span, lexer);
                let ((second_line, second_col), (_, _)) = lexer.line_col(self.node_id);
                let second_highlight = ScopeStack::form_string_highlight(self.node_id, lexer);
                return Err(CompilerError::SemanticErrorWrongTypeArgs {
                    id,
                    first_line,
                    first_col,
                    first_highlight,
                    second_line,
                    second_col,
                    second_highlight,
                });
            }
        }
        
        let current_pos_reg = code.new_register();
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::AddI(Register::Rpc, 5, current_pos_reg))));
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
            current_pos_reg,
            Register::Rsp,
            0,
        ))));
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
            Register::Rsp,
            Register::Rsp,
            4,
        ))));
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
            Register::Rfp,
            Register::Rsp,
            8,
        ))));
        let fn_label = match code.get_fn_label(&id) {
            Some(label_ref) => label_ref.clone(),
            None => return Err(CompilerError::SanityError(format!("code.get_fn_label() found nothing on FnCall.evaluate_node() for function \"{}\"", id)))
        };
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::JumpI(fn_label))));

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };
        let return_val_reg = code.new_register();
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(Register::Rsp, RETURN_VAL_OFFSET, return_val_reg))));

        Ok(Some(SymbolType::Int(IntValue::Temp(return_val_reg))))
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let condition_symbol =
            self.condition
                .evaluate_node(code, stack, lexer)?
                .ok_or(CompilerError::SanityError(format!(
                    "condition has no SymbolType (on If.evaluate_node())"
                )))?;
        condition_symbol.to_bool(self.node_id, lexer)?;

        let before_true_label = code.new_label();
        let after_true_label = code.new_label();

        let jump_if_true_voucher = code.generate_promise();
        code.push_code(CodeLine::Promise(jump_if_true_voucher));

        code.push_code(CodeLine::Deliver(Instruction::Labeled(before_true_label, Operation::Nop)));

        let promise_payment = match condition_symbol {
            SymbolType::Bool(BoolValue::Literal(boolean)) => {if boolean {
                vec![Instruction::Unlabeled(Operation::Nop)]
            } else {
                vec![Instruction::Unlabeled(Operation::JumpI(after_true_label))]
            }},
            SymbolType::Int(IntValue::Literal(number)) => {if number != 0 {
                vec![Instruction::Unlabeled(Operation::Nop)]
            } else {
                vec![Instruction::Unlabeled(Operation::JumpI(after_true_label))]
            }},
            SymbolType::Bool(BoolValue::Temp(register)) | SymbolType::Int(IntValue::Temp(register)) => {
                vec![Instruction::Unlabeled(Operation::Cbr(register, before_true_label, after_true_label))]
            },
            SymbolType::Int(IntValue::Memory(register, offset)) => {
                let new_register = code.new_register();
                vec![Instruction::Unlabeled(Operation::LoadAI(register, offset as i32, new_register)),
                    Instruction::Unlabeled(Operation::Cbr(new_register, before_true_label, after_true_label))]
            },
            _ => return Err(CompilerError::IlocErrorUndefinedBehavior(format!("condition.evaluate_node() returned unsuported type for If.evaluate(): {:?}", condition_symbol)))
        };
        code.pay_promise(jump_if_true_voucher, promise_payment);

        self.consequence.evaluate_node(code, stack, lexer)?;
        code.push_code(CodeLine::Deliver(Instruction::Labeled(after_true_label, Operation::Nop)));

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let condition_symbol =
            self.condition
                .evaluate_node(code, stack, lexer)?
                .ok_or(CompilerError::SanityError(format!(
                    "condition has no SymbolType (on IfElse.evaluate_node())"
                )))?;
        condition_symbol.to_bool(self.node_id, lexer)?;

        let before_true_label = code.new_label();
        let between_true_and_false_label = code.new_label();
        let after_if_false_label = code.new_label();

        let jump_if_true_voucher = code.generate_promise();
        code.push_code(CodeLine::Promise(jump_if_true_voucher));

        code.push_code(CodeLine::Deliver(Instruction::Labeled(before_true_label, Operation::Nop)));

        let promise_payment = match condition_symbol {
            SymbolType::Bool(BoolValue::Literal(boolean)) => {if boolean {
                vec![Instruction::Unlabeled(Operation::Nop)]
            } else {
                vec![Instruction::Unlabeled(Operation::JumpI(between_true_and_false_label))]
            }},
            SymbolType::Int(IntValue::Literal(number)) => {if number != 0 {
                vec![Instruction::Unlabeled(Operation::Nop)]
            } else {
                vec![Instruction::Unlabeled(Operation::JumpI(between_true_and_false_label))]
            }},
            SymbolType::Bool(BoolValue::Temp(register)) | SymbolType::Int(IntValue::Temp(register)) => {
                vec![Instruction::Unlabeled(Operation::Cbr(register, before_true_label, between_true_and_false_label))]
            },
            SymbolType::Int(IntValue::Memory(register, offset)) => {
                let new_register = code.new_register();
                vec![Instruction::Unlabeled(Operation::LoadAI(register, offset as i32, new_register)),
                    Instruction::Unlabeled(Operation::Cbr(new_register, before_true_label, between_true_and_false_label))]
            },
            _ => return Err(CompilerError::IlocErrorUndefinedBehavior(format!("condition.evaluate_node() returned unsuported type for IfElse.evaluate(): {:?}", condition_symbol)))
        };
        code.pay_promise(jump_if_true_voucher, promise_payment);

        self.if_true.evaluate_node(code, stack, lexer)?;

        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::JumpI(after_if_false_label))));
        code.push_code(CodeLine::Deliver(Instruction::Labeled(between_true_and_false_label, Operation::Nop)));

        self.if_false.evaluate_node(code, stack, lexer)?;

        code.push_code(CodeLine::Deliver(Instruction::Labeled(after_if_false_label, Operation::Nop)));

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let _count_init_mem_loc = self.count_init.evaluate_node(code, stack, lexer)?;

        let pre_check_label = code.new_label();
        code.push_code(CodeLine::Deliver(Instruction::Labeled(pre_check_label, Operation::Nop)));

        let count_check_symbol = self.count_check.evaluate_node(code, stack, lexer)?.ok_or(
            CompilerError::SanityError(format!(
                "count_check has no SymbolType (on For.evaluate_node())"
            )),
        )?;
        count_check_symbol.to_bool(self.node_id, lexer)?;

        let after_check_label = code.new_label();
        let escape_label = code.new_label();

        let due_instructions = match count_check_symbol {
            SymbolType::Bool(BoolValue::Literal(boolean)) => {if boolean {
                vec![Instruction::Unlabeled(Operation::Nop)]
            } else {
                vec![Instruction::Unlabeled(Operation::JumpI(escape_label))]
            }},
            SymbolType::Int(IntValue::Literal(number)) => {if number != 0 {
                vec![Instruction::Unlabeled(Operation::Nop)]
            } else {
                vec![Instruction::Unlabeled(Operation::JumpI(escape_label))]
            }},
            SymbolType::Bool(BoolValue::Temp(register)) | SymbolType::Int(IntValue::Temp(register)) => {
                vec![Instruction::Unlabeled(Operation::Cbr(register, after_check_label, escape_label))]
            },
            SymbolType::Int(IntValue::Memory(register, offset)) => {
                let new_register = code.new_register();
                vec![Instruction::Unlabeled(Operation::LoadAI(register, offset as i32, new_register)),
                    Instruction::Unlabeled(Operation::Cbr(new_register, after_check_label, escape_label))]
            },
            _ => return Err(CompilerError::IlocErrorUndefinedBehavior(format!("count_check.evaluate_node() returned unsuported type for For.evaluate(): {:?}", count_check_symbol)))
        };
        for instruction in due_instructions {
            code.push_code(CodeLine::Deliver(instruction));
        }

        code.push_code(CodeLine::Deliver(Instruction::Labeled(after_check_label, Operation::Nop)));

        self.actions.evaluate_node(code, stack, lexer)?;

        let _count_iter_mem_loc = self.count_iter.evaluate_node(code, stack, lexer)?;

        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::JumpI(pre_check_label))));
        code.push_code(CodeLine::Deliver(Instruction::Labeled(escape_label, Operation::Nop)));

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let pre_check_label = code.new_label();
        code.push_code(CodeLine::Deliver(Instruction::Labeled(pre_check_label, Operation::Nop)));

        let condition_check_symbol =
            self.condition
                .evaluate_node(code, stack, lexer)?
                .ok_or(CompilerError::SanityError(format!(
                    "condition has no SymbolType (on While.evaluate_node())"
                )))?;
                condition_check_symbol.to_bool(self.node_id, lexer)?;

        let after_check_label = code.new_label();
        let escape_label = code.new_label();

        let due_instructions = match condition_check_symbol {
            SymbolType::Bool(BoolValue::Literal(boolean)) => {if boolean {
                vec![Instruction::Unlabeled(Operation::Nop)]
            } else {
                vec![Instruction::Unlabeled(Operation::JumpI(escape_label))]
            }},
            SymbolType::Int(IntValue::Literal(number)) => {if number != 0 {
                vec![Instruction::Unlabeled(Operation::Nop)]
            } else {
                vec![Instruction::Unlabeled(Operation::JumpI(escape_label))]
            }},
            SymbolType::Bool(BoolValue::Temp(register)) | SymbolType::Int(IntValue::Temp(register)) => {
                vec![Instruction::Unlabeled(Operation::Cbr(register, after_check_label, escape_label))]
            },
            SymbolType::Int(IntValue::Memory(register, offset)) => {
                let new_register = code.new_register();
                vec![Instruction::Unlabeled(Operation::LoadAI(register, offset as i32, new_register)),
                    Instruction::Unlabeled(Operation::Cbr(new_register, after_check_label, escape_label))]
            },
            _ => return Err(CompilerError::IlocErrorUndefinedBehavior(format!("count_check.evaluate_node() returned unsuported type for While.evaluate(): {:?}", condition_check_symbol)))
        };
        for instruction in due_instructions {
            code.push_code(CodeLine::Deliver(instruction));
        }

        code.push_code(CodeLine::Deliver(Instruction::Labeled(after_check_label, Operation::Nop)));

        self.consequence.evaluate_node(code, stack, lexer)?;

        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::JumpI(pre_check_label))));
        code.push_code(CodeLine::Deliver(Instruction::Labeled(escape_label, Operation::Nop)));

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
        self.node_id
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        &self.next
    }
}

#[derive(Debug)]
pub struct CommandBlock {
    pub node_id: Span,
    pub first_command: Option<Box<dyn AstNode>>,
    pub next: Option<Box<dyn AstNode>>,
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        if let Some(command) = &self.first_command {
            stack.add_scope(None);
            command.evaluate_node(code, stack, lexer)?;
            stack.remove_scope()?;
        };

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(None)
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let condition_symbol =
            self.condition
                .evaluate_node(code, stack, lexer)?
                .ok_or(CompilerError::SanityError(format!(
                    "condition has no SymbolType (on Ternary.evaluate_node())"
                )))?;
        let if_true_symbol =
            self.if_true
                .evaluate_node(code, stack, lexer)?
                .ok_or(CompilerError::SanityError(format!(
                    "if_true has no SymbolType (on Ternary.evaluate_node())"
                )))?;
        let if_false_symbol =
            self.if_false
                .evaluate_node(code, stack, lexer)?
                .ok_or(CompilerError::SanityError(format!(
                    "if_false has no SymbolType (on Ternary.evaluate_node())"
                )))?;
        let return_symbol = match condition_symbol.to_bool(self.left_span, lexer)? {
            BoolValue::Literal(truthy_value) => {
                if truthy_value {
                    Ok(Some(if_true_symbol))
                } else {
                    Ok(Some(if_false_symbol))
                }
            }
            _ => Ok(Some(if_true_symbol.associate_with(
                &if_false_symbol,
                self.right_span,
                lexer,
            )?)),
        };

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        return_symbol
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
    ) -> Result<SymbolType, CompilerError> {
        match &self.op_type {
            BinaryType::BoolOr => match (
                left_value.to_bool(self.node_id, lexer)?,
                right_value.to_bool(self.node_id, lexer)?,
            ) {
                (BoolValue::Undefined, _) | (_, BoolValue::Undefined) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                (BoolValue::Literal(left_value), BoolValue::Literal(right_value)) => {
                    Ok(SymbolType::Bool(BoolValue::Literal(left_value || right_value)))
                }
                (BoolValue::Temp(register), BoolValue::Literal(value)) | (BoolValue::Literal(value), BoolValue::Temp(register)) => {
                    if value {
                        Ok(SymbolType::Bool(BoolValue::Literal(true)))
                    } else {
                        Ok(SymbolType::Bool(BoolValue::Temp(register)))
                    }
                }
                (BoolValue::Temp(left_register), BoolValue::Temp(right_register)) => {
                    code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Or(
                        left_register,
                        right_register,
                    ))));
                    Ok(SymbolType::Bool(BoolValue::Temp(left_register)))
                }
            },
            BinaryType::BoolAnd => match (
                left_value.to_bool(self.node_id, lexer)?,
                right_value.to_bool(self.node_id, lexer)?,
            ) {
                (BoolValue::Undefined, _) | (_, BoolValue::Undefined) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                (BoolValue::Literal(left_value), BoolValue::Literal(right_value)) => {
                    Ok(SymbolType::Bool(BoolValue::Literal(left_value && right_value)))
                }
                (BoolValue::Temp(register), BoolValue::Literal(value)) | (BoolValue::Literal(value), BoolValue::Temp(register)) => {
                    if !value {
                        Ok(SymbolType::Bool(BoolValue::Literal(false)))
                    } else {
                        Ok(SymbolType::Bool(BoolValue::Temp(register)))
                    }
                }
                (BoolValue::Temp(left_register), BoolValue::Temp(right_register)) => {
                    code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::And(
                        left_register,
                        right_register,
                    ))));
                    Ok(SymbolType::Bool(BoolValue::Temp(left_register)))
                }
            },
            BinaryType::BitOr => match (
                left_value.to_int(self.node_id, lexer)?,
                right_value.to_int(self.node_id, lexer)?,
            ) {
                (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                    Ok(SymbolType::Int(IntValue::Literal(left_value | right_value)))
                }
                (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
            },
            BinaryType::BitXor => match (
                left_value.to_int(self.node_id, lexer)?,
                right_value.to_int(self.node_id, lexer)?,
            ) {
                (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                    Ok(SymbolType::Int(IntValue::Literal(left_value ^ right_value)))
                }
                (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
            },
            BinaryType::BitAnd => match (
                left_value.to_int(self.node_id, lexer)?,
                right_value.to_int(self.node_id, lexer)?,
            ) {
                (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                    Ok(SymbolType::Int(IntValue::Literal(left_value & right_value)))
                }
                (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
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
                    SymbolType::Bool(_) => {
                        match (
                            left_value.to_int(self.node_id, lexer)?,
                            right_value.to_int(self.node_id, lexer)?,
                        ) {
                            (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                                Ok(SymbolType::Int(IntValue::Literal(left_value + right_value)))
                            }
                            (_, _) => Ok(SymbolType::Int(IntValue::Undefined)),
                        }
                    }
                    SymbolType::Int(_) => match (left_value, right_value) {
                        (SymbolType::Int(left_value), SymbolType::Int(right_value)) => match (left_value, right_value) {
                            bad @ (IntValue::Undefined, _)
                            | bad @ (_, IntValue::Undefined) => {
                                Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                                    "Binary operation Add matched undefined with something else: {:?}",
                                    bad
                                )))
                            }
                            (IntValue::Temp(register), IntValue::Literal(other_value)) | (IntValue::Literal(other_value), IntValue::Temp(register)) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::AddI(
                                    register,
                                    other_value,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Temp(register), IntValue::Memory(other_register, offset)) | (IntValue::Memory(other_register, offset), IntValue::Temp(register)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Add(
                                    register,
                                    new_register,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Literal(value), IntValue::Memory(mem_register, offset)) | (IntValue::Memory(mem_register, offset), IntValue::Literal(value)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::AddI(
                                    new_register,
                                    value,
                                    new_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(new_register)))
                            }
                            (
                                IntValue::Literal(left_value),
                                IntValue::Literal(right_value),
                            ) => Ok(SymbolType::Int(IntValue::Literal(left_value + right_value))),
                            (
                                IntValue::Memory(mem_left_register, left_offset),
                                IntValue::Memory(mem_right_register, right_offset),
                            ) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_left_register,
                                    left_offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_right_register,
                                    right_offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Add(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                            (
                                IntValue::Temp(left_register),
                                IntValue::Temp(right_register),
                            ) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Add(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                        }
                        bad => Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                            "Binary operation Add with unsuported types: {:?}",
                            bad
                        ))),
                    },
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
            BinaryType::Sub => {
                match left_value.associate_with(&right_value, self.node_id, lexer)? {
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
                    SymbolType::Bool(_) => {
                        match (
                            left_value.to_int(self.node_id, lexer)?,
                            right_value.to_int(self.node_id, lexer)?,
                        ) {
                            (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                                Ok(SymbolType::Int(IntValue::Literal(left_value - right_value)))
                            }
                            (_, _) => Ok(SymbolType::Int(IntValue::Undefined)),
                        }
                    }
                    SymbolType::Int(_) => match (left_value, right_value) {
                        (SymbolType::Int(left_value), SymbolType::Int(right_value)) => match (left_value, right_value) {
                            bad @ (IntValue::Undefined, _)
                            | bad @ (_, IntValue::Undefined) => {
                                Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                                    "Binary operation Sub matched undefined with something else: {:?}",
                                    bad
                                )))
                            }
                            (IntValue::Temp(register), IntValue::Literal(other_value)) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::SubI(
                                    register,
                                    other_value,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Literal(other_value), IntValue::Temp(register)) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::MultI(
                                    register,
                                    -1,
                                    register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::AddI(
                                    register,
                                    other_value,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register))) // 1 - a = -a + 1
                            }
                            (IntValue::Temp(register), IntValue::Memory(other_register, offset)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Sub(
                                    register,
                                    new_register,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Memory(other_register, offset), IntValue::Temp(register)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Sub(
                                    new_register,
                                    register,
                                    new_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(new_register)))
                            }
                            (IntValue::Literal(value), IntValue::Memory(mem_register, offset)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::MultI(
                                    new_register,
                                    -1,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::AddI(
                                    new_register,
                                    value,
                                    new_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(new_register))) // 1 - a = -a + 1
                            }
                            (IntValue::Memory(mem_register, offset), IntValue::Literal(value)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::SubI(
                                    new_register,
                                    value,
                                    new_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(new_register)))
                            }
                            (
                                IntValue::Literal(left_value),
                                IntValue::Literal(right_value),
                            ) => Ok(SymbolType::Int(IntValue::Literal(left_value - right_value))),
                            (
                                IntValue::Memory(mem_left_register, left_offset),
                                IntValue::Memory(mem_right_register, right_offset),
                            ) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_left_register,
                                    left_offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_right_register,
                                    right_offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Sub(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                            (
                                IntValue::Temp(left_register),
                                IntValue::Temp(right_register),
                            ) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Sub(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                        }
                        bad => Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                            "Binary operation Sub with unsuported types: {:?}",
                            bad
                        ))),
                    },
                    SymbolType::Float(_) => match (
                        left_value.to_float(self.node_id, lexer)?,
                        right_value.to_float(self.node_id, lexer)?,
                    ) {
                        (Some(left_value), Some(right_value)) => {
                            Ok(SymbolType::Float(Some(left_value - right_value)))
                        }
                        (_, _) => Ok(SymbolType::Float(None)),
                    },
                }
            }
            BinaryType::Mult => {
                match left_value.associate_with(&right_value, self.node_id, lexer)? {
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
                    SymbolType::Bool(_) => {
                        match (
                            left_value.to_int(self.node_id, lexer)?,
                            right_value.to_int(self.node_id, lexer)?,
                        ) {
                            (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                                Ok(SymbolType::Int(IntValue::Literal(left_value * right_value)))
                            }
                            (_, _) => Ok(SymbolType::Int(IntValue::Undefined)),
                        }
                    },
                    SymbolType::Int(_) => match (left_value, right_value) {
                        (SymbolType::Int(left_value), SymbolType::Int(right_value)) => match (left_value, right_value) {
                            bad @ (IntValue::Undefined, _)
                            | bad @ (_, IntValue::Undefined) => {
                                Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                                    "Binary operation Mult matched undefined with something else: {:?}",
                                    bad
                                )))
                            }
                            (IntValue::Temp(register), IntValue::Literal(other_value)) | (IntValue::Literal(other_value), IntValue::Temp(register)) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::MultI(
                                    register,
                                    other_value,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Temp(register), IntValue::Memory(other_register, offset)) | (IntValue::Memory(other_register, offset), IntValue::Temp(register)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Mult(
                                    register,
                                    new_register,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Literal(value), IntValue::Memory(mem_register, offset)) | (IntValue::Memory(mem_register, offset), IntValue::Literal(value)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::MultI(
                                    new_register,
                                    value,
                                    new_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(new_register)))
                            }
                            (
                                IntValue::Literal(left_value),
                                IntValue::Literal(right_value),
                            ) => Ok(SymbolType::Int(IntValue::Literal(left_value * right_value))),
                            (
                                IntValue::Memory(mem_left_register, left_offset),
                                IntValue::Memory(mem_right_register, right_offset),
                            ) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_left_register,
                                    left_offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_right_register,
                                    right_offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Mult(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                            (
                                IntValue::Temp(left_register),
                                IntValue::Temp(right_register),
                            ) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Mult(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                        }
                        bad => Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                            "Binary operation Mult with unsuported types: {:?}",
                            bad
                        ))),
                    },
                    SymbolType::Float(_) => match (
                        left_value.to_float(self.node_id, lexer)?,
                        right_value.to_float(self.node_id, lexer)?,
                    ) {
                        (Some(left_value), Some(right_value)) => {
                            Ok(SymbolType::Float(Some(left_value * right_value)))
                        }
                        (_, _) => Ok(SymbolType::Float(None)),
                    },
                }
            }
            BinaryType::Div => {
                match left_value.associate_with(&right_value, self.node_id, lexer)? {
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
                    SymbolType::Bool(_) => {
                        match (
                            left_value.to_int(self.node_id, lexer)?,
                            right_value.to_int(self.node_id, lexer)?,
                        ) {
                            (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                                if right_value == 0 {
                                    Ok(SymbolType::Int(IntValue::Literal(0i32)))
                                } else {
                                    Ok(SymbolType::Int(IntValue::Literal(left_value / right_value)))
                                }
                            }
                            (_, _) => Ok(SymbolType::Int(IntValue::Undefined)),
                        }
                    }
                    SymbolType::Int(_) => match (left_value, right_value) {
                        (SymbolType::Int(left_value), SymbolType::Int(right_value)) => match (left_value, right_value) {
                            bad @ (IntValue::Undefined, _)
                            | bad @ (_, IntValue::Undefined) => {
                                Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                                    "Binary operation Div matched undefined with something else: {:?}",
                                    bad
                                )))
                            }
                            (IntValue::Temp(register), IntValue::Literal(other_value)) => {
                                if other_value == 0 {
                                    return Err(CompilerError::IlocErrorUndefinedBehavior(format!("Division of expression by literal 0")))
                                }
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::DivI(
                                    register,
                                    other_value,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Literal(other_value), IntValue::Temp(register)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    other_value,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Div(
                                    new_register,
                                    register,
                                    new_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Temp(register), IntValue::Memory(other_register, offset)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Div(
                                    register,
                                    new_register,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Memory(other_register, offset), IntValue::Temp(register)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Div(
                                    new_register,
                                    register,
                                    new_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(new_register)))
                            }
                            (IntValue::Literal(value), IntValue::Memory(mem_register, offset)) => {
                                let lit_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    value,
                                    lit_register,
                                ))));
                                let other_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    other_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Div(
                                    lit_register,
                                    other_register,
                                    lit_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(lit_register)))
                            }
                            (IntValue::Memory(mem_register, offset), IntValue::Literal(value)) => {
                                if value == 0 {
                                    return Err(CompilerError::IlocErrorUndefinedBehavior(format!("Division of variable by literal 0")))
                                }
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::DivI(
                                    new_register,
                                    value,
                                    new_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(new_register)))
                            }
                            (
                                IntValue::Literal(left_value),
                                IntValue::Literal(right_value),
                            ) => {
                                if right_value == 0 {
                                    Err(CompilerError::IlocErrorUndefinedBehavior(format!("Division by literal 0: \"{} / 0\"", left_value)))
                                } else {
                                    Ok(SymbolType::Int(IntValue::Literal(left_value / right_value)))
                                }
                                
                            },
                            (
                                IntValue::Memory(mem_left_register, left_offset),
                                IntValue::Memory(mem_right_register, right_offset),
                            ) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_left_register,
                                    left_offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_right_register,
                                    right_offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Div(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                            (
                                IntValue::Temp(left_register),
                                IntValue::Temp(right_register),
                            ) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Div(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                        }
                        bad => Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                            "Binary operation Div with unsuported types: {:?}",
                            bad
                        ))),
                    },
                    SymbolType::Float(_) => match (
                        left_value.to_float(self.node_id, lexer)?,
                        right_value.to_float(self.node_id, lexer)?,
                    ) {
                        (Some(left_value), Some(right_value)) => {
                            if right_value == 0.0 {
                                Ok(SymbolType::Float(Some(0f64)))
                            } else {
                                Ok(SymbolType::Float(Some(left_value / right_value)))
                            }
                        }
                        (_, _) => Ok(SymbolType::Float(None)),
                    },
                }
            }
            BinaryType::Mod => {
                match left_value.associate_with(&right_value, self.node_id, lexer)? {
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
                            (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                                if right_value == 0 {
                                    Ok(SymbolType::Int(IntValue::Literal(0i32)))
                                } else {
                                    Ok(SymbolType::Int(IntValue::Literal(left_value % right_value)))
                                }
                            }
                            (_, _) => Ok(SymbolType::Int(IntValue::Undefined)),
                        }
                    }
                    SymbolType::Float(_) => match (
                        left_value.to_float(self.node_id, lexer)?,
                        right_value.to_float(self.node_id, lexer)?,
                    ) {
                        (Some(left_value), Some(right_value)) => {
                            if right_value == 0.0 {
                                Ok(SymbolType::Float(Some(0f64)))
                            } else {
                                Ok(SymbolType::Float(Some(left_value % right_value)))
                            }
                        }
                        (_, _) => Ok(SymbolType::Float(None)),
                    },
                }
            }
            BinaryType::Equal => {
                match left_value.associate_with(&right_value, self.node_id, lexer)? {
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
                    SymbolType::Bool(_) => {
                        match (
                            left_value.to_int(self.node_id, lexer)?,
                            right_value.to_int(self.node_id, lexer)?,
                        ) {
                            (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                                Ok(SymbolType::Bool(BoolValue::Literal(left_value == right_value)))
                            }
                            (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                        }
                    }
                    SymbolType::Int(_) => match (left_value, right_value) {
                        (SymbolType::Int(left_value), SymbolType::Int(right_value)) => match (left_value, right_value) {
                            bad @ (IntValue::Undefined, _)
                            | bad @ (_, IntValue::Undefined) => {
                                Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                                    "Binary operation Equal matched undefined with something else: {:?}",
                                    bad
                                )))
                            }
                            (IntValue::Temp(register), IntValue::Literal(other_value)) | (IntValue::Literal(other_value), IntValue::Temp(register)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    other_value,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpEQ(
                                    register,
                                    new_register,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Temp(register), IntValue::Memory(other_register, offset)) | (IntValue::Memory(other_register, offset), IntValue::Temp(register)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpEQ(
                                    register,
                                    new_register,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Literal(value), IntValue::Memory(mem_register, offset)) | (IntValue::Memory(mem_register, offset), IntValue::Literal(value)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                let lit_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    value,
                                    lit_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpEQ(
                                    new_register,
                                    lit_register,
                                    new_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(new_register)))
                            }
                            (
                                IntValue::Literal(left_value),
                                IntValue::Literal(right_value),
                            ) => Ok(SymbolType::Bool(BoolValue::Literal(left_value == right_value))),
                            (
                                IntValue::Memory(mem_left_register, left_offset),
                                IntValue::Memory(mem_right_register, right_offset),
                            ) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_left_register,
                                    left_offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_right_register,
                                    right_offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpEQ(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                            (
                                IntValue::Temp(left_register),
                                IntValue::Temp(right_register),
                            ) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpEQ(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                        }
                        bad => Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                            "Binary operation Equal with unsuported types: {:?}",
                            bad
                        ))),
                    },
                    SymbolType::Float(_) => match (
                        left_value.to_float(self.node_id, lexer)?,
                        right_value.to_float(self.node_id, lexer)?,
                    ) {
                        (Some(left_value), Some(right_value)) => {
                            Ok(SymbolType::Bool(BoolValue::Literal(left_value == right_value)))
                        }
                        (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                    },
                }
            }
            BinaryType::NotEqual => {
                match left_value.associate_with(&right_value, self.node_id, lexer)? {
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
                    SymbolType::Bool(_) => {
                        match (
                            left_value.to_int(self.node_id, lexer)?,
                            right_value.to_int(self.node_id, lexer)?,
                        ) {
                            (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                                Ok(SymbolType::Bool(BoolValue::Literal(left_value != right_value)))
                            }
                            (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                        }
                    }
                    SymbolType::Int(_) => match (left_value, right_value) {
                        (SymbolType::Int(left_value), SymbolType::Int(right_value)) => match (left_value, right_value) {
                            bad @ (IntValue::Undefined, _)
                            | bad @ (_, IntValue::Undefined) => {
                                Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                                    "Binary operation NotEqual matched undefined with something else: {:?}",
                                    bad
                                )))
                            }
                            (IntValue::Temp(register), IntValue::Literal(other_value)) | (IntValue::Literal(other_value), IntValue::Temp(register)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    other_value,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpNE(
                                    register,
                                    new_register,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Temp(register), IntValue::Memory(other_register, offset)) | (IntValue::Memory(other_register, offset), IntValue::Temp(register)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpNE(
                                    register,
                                    new_register,
                                    register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(register)))
                            }
                            (IntValue::Literal(value), IntValue::Memory(mem_register, offset)) | (IntValue::Memory(mem_register, offset), IntValue::Literal(value)) => {
                                let new_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    new_register,
                                ))));
                                let lit_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    value,
                                    lit_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpNE(
                                    new_register,
                                    lit_register,
                                    new_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(new_register)))
                            }
                            (
                                IntValue::Literal(left_value),
                                IntValue::Literal(right_value),
                            ) => Ok(SymbolType::Bool(BoolValue::Literal(left_value != right_value))),
                            (
                                IntValue::Memory(mem_left_register, left_offset),
                                IntValue::Memory(mem_right_register, right_offset),
                            ) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_left_register,
                                    left_offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_right_register,
                                    right_offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpNE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                            (
                                IntValue::Temp(left_register),
                                IntValue::Temp(right_register),
                            ) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpNE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                        }
                        bad => Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                            "Binary operation NotEqual with unsuported types: {:?}",
                            bad
                        ))),
                    },
                    SymbolType::Float(_) => match (
                        left_value.to_float(self.node_id, lexer)?,
                        right_value.to_float(self.node_id, lexer)?,
                    ) {
                        (Some(left_value), Some(right_value)) => {
                            Ok(SymbolType::Bool(BoolValue::Literal(left_value != right_value)))
                        }
                        (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                    },
                }
            }
            BinaryType::Lesser => {
                match left_value.associate_with(&right_value, self.node_id, lexer)? {
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
                    SymbolType::Bool(_) => {
                        match (
                            left_value.to_int(self.node_id, lexer)?,
                            right_value.to_int(self.node_id, lexer)?,
                        ) {
                            (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                                Ok(SymbolType::Bool(BoolValue::Literal(left_value < right_value)))
                            }
                            (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                        }
                    }
                    SymbolType::Int(_) => match (left_value, right_value) {
                        (SymbolType::Int(left_value), SymbolType::Int(right_value)) => match (left_value, right_value) {
                            bad @ (IntValue::Undefined, _)
                            | bad @ (_, IntValue::Undefined) => {
                                Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                                    "Binary operation Lesser matched undefined with something else: {:?}",
                                    bad
                                )))
                            }
                            (IntValue::Temp(left_register), IntValue::Literal(other_value)) => {
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    other_value,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Literal(other_value), IntValue::Temp(right_register)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    other_value,
                                    left_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Temp(left_register), IntValue::Memory(other_register, offset)) => {
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Memory(other_register, offset), IntValue::Temp(right_register)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    left_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Literal(value), IntValue::Memory(mem_register, offset)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    value,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Memory(mem_register, offset), IntValue::Literal(value)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    value,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (
                                IntValue::Literal(left_value),
                                IntValue::Literal(right_value),
                            ) => Ok(SymbolType::Bool(BoolValue::Literal(left_value < right_value))),
                            (
                                IntValue::Memory(mem_left_register, left_offset),
                                IntValue::Memory(mem_right_register, right_offset),
                            ) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_left_register,
                                    left_offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_right_register,
                                    right_offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                            (
                                IntValue::Temp(left_register),
                                IntValue::Temp(right_register),
                            ) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                        }
                        bad => Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                            "Binary operation Lesser with unsuported types: {:?}",
                            bad
                        ))),
                    },
                    SymbolType::Float(_) => match (
                        left_value.to_float(self.node_id, lexer)?,
                        right_value.to_float(self.node_id, lexer)?,
                    ) {
                        (Some(left_value), Some(right_value)) => {
                            Ok(SymbolType::Bool(BoolValue::Literal(left_value < right_value)))
                        }
                        (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                    },
                }
            }
            BinaryType::Greater => {
                match left_value.associate_with(&right_value, self.node_id, lexer)? {
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
                    SymbolType::Bool(_) => {
                        match (
                            left_value.to_int(self.node_id, lexer)?,
                            right_value.to_int(self.node_id, lexer)?,
                        ) {
                            (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                                Ok(SymbolType::Bool(BoolValue::Literal(left_value > right_value)))
                            }
                            (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                        }
                    }
                    SymbolType::Int(_) => match (left_value, right_value) {
                        (SymbolType::Int(left_value), SymbolType::Int(right_value)) => match (left_value, right_value) {
                            bad @ (IntValue::Undefined, _)
                            | bad @ (_, IntValue::Undefined) => {
                                Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                                    "Binary operation Greater matched undefined with something else: {:?}",
                                    bad
                                )))
                            }
                            (IntValue::Temp(left_register), IntValue::Literal(other_value)) => {
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    other_value,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Literal(other_value), IntValue::Temp(right_register)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    other_value,
                                    left_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Temp(left_register), IntValue::Memory(other_register, offset)) => {
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Memory(other_register, offset), IntValue::Temp(right_register)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    left_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Literal(value), IntValue::Memory(mem_register, offset)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    value,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Memory(mem_register, offset), IntValue::Literal(value)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    value,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (
                                IntValue::Literal(left_value),
                                IntValue::Literal(right_value),
                            ) => Ok(SymbolType::Bool(BoolValue::Literal(left_value > right_value))),
                            (
                                IntValue::Memory(mem_left_register, left_offset),
                                IntValue::Memory(mem_right_register, right_offset),
                            ) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_left_register,
                                    left_offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_right_register,
                                    right_offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                            (
                                IntValue::Temp(left_register),
                                IntValue::Temp(right_register),
                            ) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGT(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                        }
                        bad => Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                            "Binary operation Greater with unsuported types: {:?}",
                            bad
                        ))),
                    },
                    SymbolType::Float(_) => match (
                        left_value.to_float(self.node_id, lexer)?,
                        right_value.to_float(self.node_id, lexer)?,
                    ) {
                        (Some(left_value), Some(right_value)) => {
                            Ok(SymbolType::Bool(BoolValue::Literal(left_value > right_value)))
                        }
                        (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                    },
                }
            }
            BinaryType::LesserEqual => {
                match left_value.associate_with(&right_value, self.node_id, lexer)? {
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
                    SymbolType::Bool(_) => {
                        match (
                            left_value.to_int(self.node_id, lexer)?,
                            right_value.to_int(self.node_id, lexer)?,
                        ) {
                            (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                                Ok(SymbolType::Bool(BoolValue::Literal(left_value <= right_value)))
                            }
                            (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                        }
                    }
                    SymbolType::Int(_) => match (left_value, right_value) {
                        (SymbolType::Int(left_value), SymbolType::Int(right_value)) => match (left_value, right_value) {
                            bad @ (IntValue::Undefined, _)
                            | bad @ (_, IntValue::Undefined) => {
                                Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                                    "Binary operation LesserEqual matched undefined with something else: {:?}",
                                    bad
                                )))
                            }
                            (IntValue::Temp(left_register), IntValue::Literal(other_value)) => {
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    other_value,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Literal(other_value), IntValue::Temp(right_register)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    other_value,
                                    left_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Temp(left_register), IntValue::Memory(other_register, offset)) => {
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Memory(other_register, offset), IntValue::Temp(right_register)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    left_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Literal(value), IntValue::Memory(mem_register, offset)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    value,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Memory(mem_register, offset), IntValue::Literal(value)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    value,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (
                                IntValue::Literal(left_value),
                                IntValue::Literal(right_value),
                            ) => Ok(SymbolType::Bool(BoolValue::Literal(left_value <= right_value))),
                            (
                                IntValue::Memory(mem_left_register, left_offset),
                                IntValue::Memory(mem_right_register, right_offset),
                            ) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_left_register,
                                    left_offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_right_register,
                                    right_offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                            (
                                IntValue::Temp(left_register),
                                IntValue::Temp(right_register),
                            ) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpLE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                        }
                        bad => Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                            "Binary operation LesserEqual with unsuported types: {:?}",
                            bad
                        ))),
                    },
                    SymbolType::Float(_) => match (
                        left_value.to_float(self.node_id, lexer)?,
                        right_value.to_float(self.node_id, lexer)?,
                    ) {
                        (Some(left_value), Some(right_value)) => {
                            Ok(SymbolType::Bool(BoolValue::Literal(left_value <= right_value)))
                        }
                        (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                    },
                }
            }
            BinaryType::GreaterEqual => {
                match left_value.associate_with(&right_value, self.node_id, lexer)? {
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
                    SymbolType::Int(_) => match (left_value, right_value) {
                        (SymbolType::Int(left_value), SymbolType::Int(right_value)) => match (left_value, right_value) {
                            bad @ (IntValue::Undefined, _)
                            | bad @ (_, IntValue::Undefined) => {
                                Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                                    "Binary operation GreaterEqual matched undefined with something else: {:?}",
                                    bad
                                )))
                            }
                            (IntValue::Temp(left_register), IntValue::Literal(other_value)) => {
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    other_value,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Literal(other_value), IntValue::Temp(right_register)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    other_value,
                                    left_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Temp(left_register), IntValue::Memory(other_register, offset)) => {
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Memory(other_register, offset), IntValue::Temp(right_register)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    other_register,
                                    offset as i32,
                                    left_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Literal(value), IntValue::Memory(mem_register, offset)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    value,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (IntValue::Memory(mem_register, offset), IntValue::Literal(value)) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_register,
                                    offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                                    value,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            }
                            (
                                IntValue::Literal(left_value),
                                IntValue::Literal(right_value),
                            ) => Ok(SymbolType::Bool(BoolValue::Literal(left_value >= right_value))),
                            (
                                IntValue::Memory(mem_left_register, left_offset),
                                IntValue::Memory(mem_right_register, right_offset),
                            ) => {
                                let left_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_left_register,
                                    left_offset as i32,
                                    left_register,
                                ))));
                                let right_register = code.new_register();
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                                    mem_right_register,
                                    right_offset as i32,
                                    right_register,
                                ))));
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                            (
                                IntValue::Temp(left_register),
                                IntValue::Temp(right_register),
                            ) => {
                                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::CmpGE(
                                    left_register,
                                    right_register,
                                    left_register,
                                ))));
                                Ok(SymbolType::Int(IntValue::Temp(left_register)))
                            },
                        }
                        bad => Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                            "Binary operation GreaterEqual with unsuported types: {:?}",
                            bad
                        ))),
                    },
                    SymbolType::Bool(_) => {
                        match (
                            left_value.to_int(self.node_id, lexer)?,
                            right_value.to_int(self.node_id, lexer)?,
                        ) {
                            (IntValue::Literal(left_value), IntValue::Literal(right_value)) => {
                                Ok(SymbolType::Bool(BoolValue::Literal(left_value >= right_value)))
                            }
                            (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                        }
                    }
                    SymbolType::Float(_) => match (
                        left_value.to_float(self.node_id, lexer)?,
                        right_value.to_float(self.node_id, lexer)?,
                    ) {
                        (Some(left_value), Some(right_value)) => {
                            Ok(SymbolType::Bool(BoolValue::Literal(left_value >= right_value)))
                        }
                        (_, _) => Ok(SymbolType::Bool(BoolValue::Undefined)),
                    },
                }
            }
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let left_value_type = match self.lhs.evaluate_node(code, stack, lexer)? {
            Some(value) => value,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "lhs.evaluate_node() returned None for Binary of type {:?}",
                    self.op_type
                )))
            }
        };

        let right_value_type = match self.rhs.evaluate_node(code, stack, lexer)? {
            Some(value) => value,
            None => {
                return Err(CompilerError::SanityError(format!(
                    "rhs.evaluate_node() returned None for Binary of type {:?}",
                    self.op_type
                )))
            }
        };

        let return_value = Ok(Some(self.binary_evaluation(
            left_value_type,
            right_value_type,
            lexer,
            code,
        )?));

        if let Some(node) = &self.next {
            return Err(CompilerError::SanityError(format!(
                "Binary {:?} has a self.next node: {:?}",
                self.op_type, node
            )));
        };

        return_value
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
    ) -> Result<SymbolType, CompilerError> {
        match &self.op_type {
            UnaryType::Positive => match type_value {
                SymbolType::Int(IntValue::Undefined) => {
                    Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                        "Unary operation Positive matched with undefined Int."
                    )))
                }
                symbol @ SymbolType::Int(_) => {
                    Ok(symbol)
                }
                symbol @ SymbolType::Float(_) => Ok(symbol),
                SymbolType::Bool(BoolValue::Literal(value)) => {
                    match value {
                        true => Ok(SymbolType::Int(IntValue::Literal(1))),
                        false => Ok(SymbolType::Int(IntValue::Literal(0))),
                    }
                },
                SymbolType::Bool(BoolValue::Temp(register)) => Ok(SymbolType::Int(IntValue::Temp(register))),
                SymbolType::Bool(BoolValue::Undefined) => Ok(SymbolType::Int(IntValue::Undefined)),
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
                SymbolType::Int(IntValue::Undefined) => {
                    Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                        "Unary operation Negative matched with undefined Int."
                    )))
                }
                SymbolType::Int(IntValue::Literal(number)) => {
                    Ok(SymbolType::Int(IntValue::Literal(-number)))
                }
                SymbolType::Int(IntValue::Temp(register)) => {
                    code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::MultI(
                        register,
                        -1,
                        register,
                    ))));
                    Ok(SymbolType::Int(IntValue::Temp(register)))
                }
                SymbolType::Int(IntValue::Memory(offset_source, offset)) => {
                    let register = code.new_register();
                    code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                        offset_source,
                        offset as i32,
                        register,
                    ))));
                    code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::MultI(
                        register,
                        -1,
                        register,
                    ))));
                    Ok(SymbolType::Int(IntValue::Temp(register)))
                }
                symbol @ SymbolType::Float(_) => Ok(symbol),
                SymbolType::Bool(maybe_value) => match &maybe_value {
                    BoolValue::Literal(value) => match value {
                        true => Ok(SymbolType::Int(IntValue::Literal(-1))),
                        false => Ok(SymbolType::Int(IntValue::Literal(0))),
                    },
                    BoolValue::Temp(register) => {
                        let register_copy = *register;
                        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::MultI(
                            register_copy,
                            -1,
                            register_copy,
                        ))));
                        Ok(SymbolType::Int(IntValue::Temp(register_copy)))
                    }
                    BoolValue::Undefined => Ok(SymbolType::Int(IntValue::Undefined)),
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
                    IntValue::Literal(value) => {
                        if *value == 0i32 {
                            Ok(SymbolType::Bool(BoolValue::Literal(true)))
                        } else {
                            Ok(SymbolType::Bool(BoolValue::Literal(false)))
                        }
                    }
                    _ => Ok(SymbolType::Bool(BoolValue::Undefined)),
                },
                SymbolType::Float(maybe_value) => match &maybe_value {
                    Some(value) => {
                        if *value == 0f64 {
                            Ok(SymbolType::Bool(BoolValue::Literal(true)))
                        } else {
                            Ok(SymbolType::Bool(BoolValue::Literal(false)))
                        }
                    }
                    None => Ok(SymbolType::Bool(BoolValue::Undefined)),
                },
                SymbolType::Bool(maybe_value) => match &maybe_value {
                    BoolValue::Literal(value) => match value {
                        true => Ok(SymbolType::Bool(BoolValue::Literal(false))),
                        false => Ok(SymbolType::Bool(BoolValue::Literal(true))),
                    },
                    BoolValue::Temp(register) => {
                        let register_copy = *register;
                        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::Not(
                            register_copy,
                        ))));
                        Ok(SymbolType::Bool(BoolValue::Temp(register_copy)))
                    }
                    BoolValue::Undefined => Ok(SymbolType::Bool(BoolValue::Undefined)),
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
                    IntValue::Literal(value) => {
                        if *value == 0i32 {
                            Ok(SymbolType::Bool(BoolValue::Literal(false)))
                        } else {
                            Ok(SymbolType::Bool(BoolValue::Literal(true)))
                        }
                    }
                    _ => Ok(SymbolType::Bool(BoolValue::Undefined)),
                },
                SymbolType::Float(maybe_value) => match &maybe_value {
                    Some(value) => {
                        if *value == 0f64 {
                            Ok(SymbolType::Bool(BoolValue::Literal(false)))
                        } else {
                            Ok(SymbolType::Bool(BoolValue::Literal(true)))
                        }
                    }
                    None => Ok(SymbolType::Bool(BoolValue::Undefined)),
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
            UnaryType::Hash => Ok(SymbolType::Int(IntValue::Undefined)),
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        if let Some(node) = &self.next {
            return Err(CompilerError::LexicalError(format!(
                "Unary struct holds a reference to a next node: {:?}",
                node
            )));
        };

        let type_value = match self.operand.evaluate_node(code, stack, lexer)? {
            Some(value) => value,
            None => {
                return Err(CompilerError::SemanticError(format!(
                    "Unary struct operand returned no value on evaluation: {:?}",
                    self.operand
                )))
            }
        };

        let type_value = self.unary_evaluation(type_value.clone(), lexer, code)?;

        if let Some(node) = &self.next {
            return Err(CompilerError::SanityError(format!(
                "Unary {:?} has a self.next node: {:?}",
                self.op_type, node
            )));
        };

        Ok(Some(type_value))
    }
    fn get_span(&self) -> Span {
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
pub struct VecAccess {
    node_id: Span,
    vec_name: Box<VecInvoke>,
    vec_index: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecAccess {
    pub fn new(
        node_id: Span,
        vec_name: Box<VecInvoke>,
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let vec_type_value =
            self.vec_name
                .evaluate_node(code, stack, lexer)?
                .ok_or(CompilerError::SanityError(format!(
                "VecAccess.evaluate_node() found no TypeValue from self.vec_name.evaluate_node()"
            )))?;

        let indexer_type_value = self.vec_index.evaluate_node(code, stack, lexer)?.ok_or(CompilerError::SanityError(format!(
            "VecAccess.evaluate_node() found no TypeValue from self.vec_index.evaluate_node()"
        )))?;

        let previous_def =
            stack.get_previous_def(self.vec_name.get_span(), lexer, SymbolClass::default_vec())?;

        let offset_register;
        match indexer_type_value {
            SymbolType::Int(IntValue::Temp(register)) => {
                offset_register = register;
            },
            SymbolType::Int(IntValue::Literal(number)) => {
                offset_register = code.new_register();
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                    number,
                    offset_register,
                ))));
            },
            SymbolType::Int(IntValue::Memory(register, offset)) => {
                offset_register = code.new_register();
                code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadAI(
                    register,
                    offset as i32,
                    offset_register,
                ))));
            },
            SymbolType::Int(IntValue::Undefined) => {
                return Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                    "VecAccess called with uninitialized index.\nVec name: {:?}\n Index: {:?}",
                    self.vec_name, self.vec_index
                )))
            }
            _ => {
                return Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                    "VecAccess called with unsuported type index.\nVec name: {:?}\n Index: {:?}",
                    self.vec_name, self.vec_index
                )))
            }
        } // foo[bar] => bar into register

        let type_size = vec_type_value.get_symbol_type_size() as i32;
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::MultI(
            offset_register,
            type_size,
            offset_register,
        )))); // foo[bar] => (bar * size of type) as register

        let offset = previous_def.offset as i32;
        code.push_code(CodeLine::Deliver(Instruction::Unlabeled(Operation::AddI(
            offset_register,
            offset,
            offset_register,
        )))); // foo[bar] => (bar * size of type + offset) as register

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(Some(SymbolType::Int(IntValue::Memory(offset_register, 0))))
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        let class = SymbolClass::default_var();
        let previous_def = stack.get_previous_def(span, lexer, class)?;
        let type_value = previous_def.type_value.clone();

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(Some(type_value))
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        let span = self.node_id;
        let class = SymbolClass::default_vec();
        let previous_def = stack.get_previous_def(span, lexer, class)?;
        let type_value = previous_def.type_value.clone();

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(Some(type_value))
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
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
        let var_type = SymbolType::Int(IntValue::Literal(var_value));

        let ((line, col), (_, _)) = lexer.line_col(span);
        let our_symbol = CallSymbol::new(id, span, line, col, var_type.clone(), class);

        stack.push_symbol(our_symbol)?;

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(Some(var_type))
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
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

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(Some(var_type))
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
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
        let var_type = SymbolType::Bool(BoolValue::Literal(var_value));

        let ((line, col), (_, _)) = lexer.line_col(span);
        let our_symbol = CallSymbol::new(id, span, line, col, var_type.clone(), class);

        stack.push_symbol(our_symbol)?;

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(Some(var_type))
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
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

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(Some(var_type))
    }
    fn get_span(&self) -> Span {
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
        code: &mut IlocCode,
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

        if let Some(node) = &self.next {
            node.evaluate_node(code, stack, lexer)?;
        };

        Ok(Some(var_type))
    }
    fn get_span(&self) -> Span {
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
