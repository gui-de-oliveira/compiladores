// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

use std::collections::HashMap;

use lrpar::{NonStreamingLexer, Span};

use super::error::CompilerError;
use super::instructions::Register;
use super::lexical_structures::Parameter;

#[derive(Debug)]
pub struct DefSymbol {
    pub id: String,
    pub span: Span,
    pub line: usize,
    pub col: usize,
    pub type_value: SymbolType,
    pub class: SymbolClass,
    pub size: Option<u32>,
    pub offset_source: Register,
    pub offset: u32,
}

impl DefSymbol {
    pub fn new(
        id: String,
        span: Span,
        line: usize,
        col: usize,
        type_value: SymbolType,
        class: SymbolClass,
        size: Option<u32>,
        offset_source: Register,
        offset: u32,
    ) -> DefSymbol {
        DefSymbol {
            id,
            span,
            line,
            col,
            type_value,
            class,
            size,
            offset_source,
            offset,
        }
    }
    pub fn cast_or_scream(
        &self,
        friend: &SymbolType,
        span: Span,
        lexer: &dyn NonStreamingLexer<u32>,
        check_string_size: bool,
    ) -> Result<DefSymbol, CompilerError> {
        match (&self.type_value, friend) {
            (SymbolType::String(_), right_type @ SymbolType::String(None)) => Ok(DefSymbol::new(
                self.id.clone(),
                self.span,
                self.line,
                self.col,
                right_type.clone(),
                self.class.clone(),
                self.size,
                self.offset_source,
                self.offset,
            )),
            (SymbolType::String(_), right_type @ SymbolType::String(Some(_))) => {
                let incoming_size = right_type.get_symbol_type_size();
                let our_size = self.size.unwrap_or(0) as u32;
                if check_string_size && our_size < incoming_size {
                    let highlight = ScopeStack::form_string_highlight(span, lexer);
                    let ((line, col), (_, _)) = lexer.line_col(span);
                    Err(CompilerError::SemanticErrorStringMax {
                        highlight,
                        line,
                        col,
                        variable_size: our_size,
                        string_size: incoming_size,
                    })
                } else {
                    Ok(DefSymbol::new(
                        self.id.clone(),
                        self.span,
                        self.line,
                        self.col,
                        self.type_value.clone(),
                        self.class.clone(),
                        Some(if check_string_size {
                            our_size
                        } else {
                            incoming_size
                        }),
                        self.offset_source,
                        self.offset,
                    ))
                }
            }
            (SymbolType::String(_), bad_type) => {
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorWrongType {
                    valid_type: "string".to_string(),
                    received_type: bad_type.to_str().to_string(),
                    highlight,
                    line,
                    col,
                })
            }
            (SymbolType::Char(_), right_type @ SymbolType::Char(_)) => Ok(DefSymbol::new(
                self.id.clone(),
                self.span,
                self.line,
                self.col,
                right_type.clone(),
                self.class.clone(),
                self.size,
                self.offset_source,
                self.offset,
            )),
            (SymbolType::Char(_), bad_type) => {
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorWrongType {
                    valid_type: "char".to_string(),
                    received_type: bad_type.to_str().to_string(),
                    highlight,
                    line,
                    col,
                })
            }
            (
                SymbolType::Float(_),
                right_type @ SymbolType::Float(_)
                | right_type @ SymbolType::Int(_)
                | right_type @ SymbolType::Bool(_),
            ) => Ok(DefSymbol::new(
                self.id.clone(),
                self.span,
                self.line,
                self.col,
                SymbolType::Float(right_type.to_float(span, lexer)?),
                self.class.clone(),
                self.size,
                self.offset_source,
                self.offset,
            )),
            (
                SymbolType::Int(_),
                right_type @ SymbolType::Float(_)
                | right_type @ SymbolType::Int(_)
                | right_type @ SymbolType::Bool(_),
            ) => Ok(DefSymbol::new(
                self.id.clone(),
                self.span,
                self.line,
                self.col,
                SymbolType::Int(right_type.to_int(span, lexer)?),
                self.class.clone(),
                self.size,
                self.offset_source,
                self.offset,
            )),
            (
                SymbolType::Bool(_),
                right_type @ SymbolType::Float(_)
                | right_type @ SymbolType::Int(_)
                | right_type @ SymbolType::Bool(_),
            ) => Ok(DefSymbol::new(
                self.id.clone(),
                self.span,
                self.line,
                self.col,
                SymbolType::Bool(right_type.to_bool(span, lexer)?),
                self.class.clone(),
                self.size,
                self.offset_source,
                self.offset,
            )),
            (
                bad_type @ SymbolType::Int(_)
                | bad_type @ SymbolType::Float(_)
                | bad_type @ SymbolType::Bool(_),
                SymbolType::Char(_),
            ) => {
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorCharToX {
                    invalid_type: bad_type.to_str().to_string(),
                    line,
                    col,
                    highlight,
                })
            }
            (
                bad_type @ SymbolType::Int(_)
                | bad_type @ SymbolType::Float(_)
                | bad_type @ SymbolType::Bool(_),
                SymbolType::String(_),
            ) => {
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorStringToX {
                    invalid_type: bad_type.to_str().to_string(),
                    line,
                    col,
                    highlight,
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct CallSymbol {
    pub id: String,
    pub span: Span,
    pub line: usize,
    pub col: usize,
    pub type_value: SymbolType,
    pub class: SymbolClass,
}

impl CallSymbol {
    pub fn new(
        id: String,
        span: Span,
        line: usize,
        col: usize,
        type_value: SymbolType,
        class: SymbolClass,
    ) -> CallSymbol {
        CallSymbol {
            id,
            span,
            line,
            col,
            type_value,
            class,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IntValue {
    Literal(i32),
    Memory(Register, u32),
    Temp(Register),
    Undefined,
}

#[derive(Clone, Debug)]
pub enum SymbolType {
    Int(IntValue),
    Float(Option<f64>),
    Char(Option<u8>),
    Bool(Option<bool>),
    String(Option<String>),
}

impl SymbolType {
    pub fn to_str(&self) -> &str {
        match self {
            SymbolType::Int(_) => "int",
            SymbolType::Float(_) => "float",
            SymbolType::Char(_) => "char",
            SymbolType::Bool(_) => "bool",
            SymbolType::String(_) => "string",
        }
    }
    pub fn from_str(str_type: &str) -> Result<SymbolType, CompilerError> {
        match str_type {
            "int" => Ok(SymbolType::Int(IntValue::Undefined)),
            "float" => Ok(SymbolType::Float(None)),
            "bool" => Ok(SymbolType::Bool(None)),
            "char" => Ok(SymbolType::Char(None)),
            "string" => Ok(SymbolType::String(None)),
            _ => Err(CompilerError::LexicalError(format!(
                "invalid type declaration: {}",
                str_type
            ))),
        }
    }
    pub fn to_bool(
        &self,
        span: Span,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<bool>, CompilerError> {
        match self {
            SymbolType::Bool(Some(value)) => Ok(Some(*value)),
            SymbolType::Int(IntValue::Literal(value)) => Ok(Some(*value != 0)),
            SymbolType::Float(Some(value)) => Ok(Some(*value != 0.0)),
            SymbolType::Char(_) => {
                let invalid_type = "boolean".to_string();
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorCharToX {
                    invalid_type,
                    line,
                    col,
                    highlight,
                })
            }
            SymbolType::String(_) => {
                let invalid_type = "boolean".to_string();
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorStringToX {
                    invalid_type,
                    line,
                    col,
                    highlight,
                })
            }
            _ => Ok(None),
        }
    }
    pub fn to_int(
        &self,
        span: Span,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<IntValue, CompilerError> {
        match &self {
            SymbolType::Int(int_type) => Ok(*int_type),
            SymbolType::Bool(Some(value)) => Ok(IntValue::Literal(*value as i32)),
            SymbolType::Float(Some(value)) => Ok(IntValue::Literal(*value as i32)),
            SymbolType::Char(_) => {
                let invalid_type = "int".to_string();
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorCharToX {
                    invalid_type,
                    line,
                    col,
                    highlight,
                })
            }
            SymbolType::String(_) => {
                let invalid_type = "int".to_string();
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorStringToX {
                    invalid_type,
                    line,
                    col,
                    highlight,
                })
            }
            _ => Ok(IntValue::Undefined),
        }
    }
    pub fn to_float(
        &self,
        span: Span,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<f64>, CompilerError> {
        match self {
            SymbolType::Float(Some(value)) => Ok(Some(*value)),
            SymbolType::Int(IntValue::Literal(value)) => Ok(Some(*value as f64)),
            SymbolType::Bool(Some(value)) => Ok(Some((*value as u32) as f64)),
            SymbolType::Char(_) => {
                let invalid_type = "float".to_string();
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorCharToX {
                    invalid_type,
                    line,
                    col,
                    highlight,
                })
            }
            SymbolType::String(_) => {
                let invalid_type = "float".to_string();
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorStringToX {
                    invalid_type,
                    line,
                    col,
                    highlight,
                })
            }
            _ => Ok(None),
        }
    }
    pub fn associate_with(
        &self,
        friend: &SymbolType,
        span: Span,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<SymbolType, CompilerError> {
        match (self, friend) {
            (SymbolType::Int(_), SymbolType::Int(_)) => Ok(SymbolType::Int(IntValue::Undefined)),
            (SymbolType::Float(_), SymbolType::Float(_)) => Ok(SymbolType::Float(None)),
            (SymbolType::Bool(_), SymbolType::Bool(_)) => Ok(SymbolType::Bool(None)),
            (SymbolType::Bool(_), SymbolType::Int(_))
            | (SymbolType::Int(_), SymbolType::Bool(_)) => Ok(SymbolType::Int(IntValue::Undefined)),
            (SymbolType::Float(_), SymbolType::Int(_))
            | (SymbolType::Int(_), SymbolType::Float(_))
            | (SymbolType::Bool(_), SymbolType::Float(_))
            | (SymbolType::Float(_), SymbolType::Bool(_)) => Ok(SymbolType::Float(None)),
            (SymbolType::String(_), SymbolType::String(_)) => Ok(SymbolType::String(None)),
            (SymbolType::Char(_), SymbolType::Char(_)) => Ok(SymbolType::Char(None)),
            (SymbolType::String(_), _) => {
                let invalid_type = "int or float".to_string();
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorStringToX {
                    invalid_type,
                    line,
                    col,
                    highlight,
                })
            }
            (SymbolType::Char(_), _) => {
                let invalid_type = "int or float".to_string();
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorCharToX {
                    invalid_type,
                    line,
                    col,
                    highlight,
                })
            }
            (_, SymbolType::String(_)) => {
                let invalid_type = "int or float".to_string();
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorStringToX {
                    invalid_type,
                    line,
                    col,
                    highlight,
                })
            }
            (_, SymbolType::Char(_)) => {
                let invalid_type = "int or float".to_string();
                let ((line, col), (_, _)) = lexer.line_col(span);
                let highlight = ScopeStack::form_string_highlight(span, lexer);
                Err(CompilerError::SemanticErrorCharToX {
                    invalid_type,
                    line,
                    col,
                    highlight,
                })
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
    pub fn get_symbol_type_size(&self) -> u32 {
        match self {
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
}

impl PartialEq for SymbolType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SymbolType::Char(_), SymbolType::Char(_))
            | (SymbolType::Int(_), SymbolType::Int(_))
            | (SymbolType::Float(_), SymbolType::Float(_))
            | (SymbolType::Bool(_), SymbolType::Bool(_))
            | (SymbolType::String(_), SymbolType::String(_)) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SymbolClass {
    Fn(Vec<Parameter>),
    Var { is_global: bool, offset: u32 },
    Vec { offset: u32 },
    Lit,
}

impl SymbolClass {
    pub fn default_var() -> SymbolClass {
        SymbolClass::Var {
            is_global: false,
            offset: 0,
        }
    }
    pub fn default_vec() -> SymbolClass {
        SymbolClass::Vec { offset: 0 }
    }
}

impl PartialEq for SymbolClass {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SymbolClass::Fn(_), SymbolClass::Fn(_))
            | (SymbolClass::Var { .. }, SymbolClass::Var { .. })
            | (SymbolClass::Vec { .. }, SymbolClass::Vec { .. })
            | (SymbolClass::Lit, SymbolClass::Lit) => true,
            _ => false,
        }
    }
}

impl SymbolClass {
    pub fn to_str(&self) -> &str {
        match self {
            SymbolClass::Fn(_) => "function",
            SymbolClass::Var { .. } => "variable",
            SymbolClass::Vec { .. } => "vector",
            SymbolClass::Lit => "literal",
        }
    }
}

pub struct ScopeStack {
    stack: Vec<(
        HashMap<String, DefSymbol>,
        Option<SymbolType>,
        Vec<CallSymbol>,
    )>,
    offsets: Vec<u32>,
}

impl ScopeStack {
    pub fn new() -> ScopeStack {
        ScopeStack {
            stack: vec![(HashMap::new(), None, vec![])],
            offsets: vec![0],
        }
    }

    pub fn add_scope(&mut self, scope_type: Option<SymbolType>) {
        self.offsets.push(0);
        self.stack.push((HashMap::new(), scope_type, vec![]))
    }

    pub fn remove_scope(&mut self) -> Result<HashMap<String, DefSymbol>, CompilerError> {
        self.offsets.pop();
        match self.stack.pop() {
            Some((def_table, _scope_type, _symbols)) => Ok(def_table),
            None => Err(CompilerError::FailedScoping),
        }
    }

    pub fn check_duplicate(
        &self,
        span: Span,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<(), CompilerError> {
        match self.stack.last() {
            Some((scope, _scope_type, _symbols)) => {
                let id = lexer.span_str(span).to_string();
                match scope.get(&id) {
                    Some(older_symbol) => {
                        let first_line = older_symbol.line;
                        let first_col = older_symbol.col;
                        let first_highlight =
                            ScopeStack::form_string_highlight(older_symbol.span, lexer);
                        let ((second_line, second_col), (_, _)) = lexer.line_col(span);
                        let second_highlight = ScopeStack::form_string_highlight(span, lexer);
                        Err(CompilerError::SemanticErrorDeclared {
                            id,
                            first_line,
                            first_col,
                            first_highlight,
                            second_line,
                            second_col,
                            second_highlight,
                        })
                    }
                    None => Ok(()),
                }
            }
            None => Err(CompilerError::FailedScoping),
        }
    }

    pub fn get_current_scope_type(&self) -> Result<SymbolType, CompilerError> {
        for (_scope, scope_type, _symbols) in self.stack.iter().rev() {
            match scope_type {
                Some(thing) => return Ok(thing.clone()),
                None => continue,
            }
        }
        Err(CompilerError::SanityError(format!(
            "get_current_scope_type() found no typed scopes: {:?}",
            self.stack
        )))
    }

    pub fn get_previous_def(
        &self,
        span: Span,
        lexer: &dyn NonStreamingLexer<u32>,
        expected_class: SymbolClass,
    ) -> Result<&DefSymbol, CompilerError> {
        let id = lexer.span_str(span).to_string();

        for (scope, _scope_type, _symbols) in self.stack.iter().rev() {
            if let Some(older_symbol) = scope.get(&id) {
                if older_symbol.class == expected_class {
                    return Ok(&older_symbol);
                } else {
                    let first_line = older_symbol.line;
                    let first_col = older_symbol.col;
                    let first_highlight =
                        ScopeStack::form_string_highlight(older_symbol.span, lexer);
                    let second_class = expected_class.to_str().to_owned();
                    let ((second_line, second_col), (_, _)) = lexer.line_col(span);
                    let second_highlight = ScopeStack::form_string_highlight(span, lexer);
                    return Err(match older_symbol.class {
                        SymbolClass::Var{ .. } => CompilerError::SemanticErrorVariable {
                            id,
                            first_line,
                            first_col,
                            first_highlight,
                            second_class,
                            second_line,
                            second_col,
                            second_highlight,
                        },
                        SymbolClass::Vec{ .. } => CompilerError::SemanticErrorVector {
                            id,
                            first_line,
                            first_col,
                            first_highlight,
                            second_class,
                            second_line,
                            second_col,
                            second_highlight,
                        },
                        SymbolClass::Fn(_) => CompilerError::SemanticErrorFunction {
                            id,
                            first_line,
                            first_col,
                            first_highlight,
                            second_class,
                            second_line,
                            second_col,
                            second_highlight,
                        },
                        SymbolClass::Lit => CompilerError::SanityError(
                            format!(
                                "get_previous_def() with \"{}\" matched a a literal: ({}, {}, {}) => ({}, {}, {}, {})",
                                id,
                                first_line,
                                first_col,
                                first_highlight,
                                second_class,
                                second_line,
                                second_col,
                                second_highlight
                            )
                        ),
                    });
                }
            }
        }
        let ((line, col), (_, _)) = lexer.line_col(span);
        let highlight = ScopeStack::form_string_highlight(span, lexer);
        Err(CompilerError::SemanticErrorUndeclared {
            id,
            line,
            col,
            highlight,
        })
    }

    pub fn get_previous_def_string_no_error(&self, id: &str) -> Option<&DefSymbol> {
        for (scope, _scope_type, _symbols) in self.stack.iter().rev() {
            if let Some(older_symbol) = scope.get(id) {
                return Some(&older_symbol);
            }
        }
        None
    }

    pub fn add_def_symbol(&mut self, addition: DefSymbol) -> Result<(), CompilerError> {
        match self.stack.last_mut() {
            Some((scope, _scope_type, _symbols)) => {
                scope.insert(addition.id.clone(), addition);
                Ok(())
            }
            None => Err(CompilerError::FailedScoping),
        }
    }

    pub fn push_symbol(&mut self, addition: CallSymbol) -> Result<(), CompilerError> {
        match self.stack.last_mut() {
            Some((_scope, _scope_type, symbols)) => {
                symbols.push(addition);
                Ok(())
            }
            None => Err(CompilerError::FailedScoping),
        }
    }

    pub fn pop_symbol(&mut self) -> Result<Option<CallSymbol>, CompilerError> {
        match self.stack.last_mut() {
            Some((_scope, _scope_type, symbols)) => Ok(symbols.pop()),
            None => Err(CompilerError::FailedScoping),
        }
    }

    pub fn add_offset(&mut self, extra_offset: u32) -> Result<(), CompilerError> {
        match self.offsets.last_mut() {
            Some(num) => {
                *num += extra_offset;
                Ok(())
            }
            None => Err(CompilerError::FailedScoping),
        }
    }

    pub fn get_offset(&self) -> Result<u32, CompilerError> {
        match self.offsets.last() {
            Some(num) => Ok(*num),
            None => Err(CompilerError::FailedScoping),
        }
    }

    pub fn form_string_highlight(span: Span, lexer: &dyn NonStreamingLexer<u32>) -> String {
        let ((_start_line, start_column), (_end_line, end_column)) = lexer.line_col(span);
        let mut lines = lexer.span_lines_str(span).lines().peekable();

        let mut output = String::new();

        let first_line = lines.next().unwrap_or("");
        output.push_str(&first_line);
        output.push('\n');

        for _i in 0..start_column - 1 {
            output.push(' ');
        }
        let end_of_first_line = match lines.peek() {
            Some(_) => first_line.len() + 1,
            None => end_column,
        };
        for _i in start_column..end_of_first_line {
            output.push('^');
        }

        loop {
            let next_line = match lines.next() {
                Some(string) => string,
                None => break,
            };
            output.push('\n');
            output.push_str(&next_line);
            let end_of_next_line = match lines.peek() {
                Some(_) => next_line.len() + 1,
                None => end_column,
            };
            for _i in 0..end_of_next_line {
                output.push('^');
            }
        }

        output
    }
}
