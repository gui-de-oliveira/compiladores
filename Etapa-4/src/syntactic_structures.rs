use std::collections::HashMap;

use super::error::CompilerError;

#[derive(Debug)]
pub struct Symbol {
    pub id: String,
    pub line: usize,
    pub col: usize,
    pub type_value: SymbolType,
}

impl Symbol {
    pub fn new(id: String, line: usize, col: usize, type_value: SymbolType) -> Symbol {
        Symbol {
            id,
            line,
            col,
            type_value,
        }
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Undefined,
    Int(Option<i32>),
    Float(Option<f32>),
    Char(Option<u8>),
    Bool(Option<bool>),
    String(Option<String>),
}

impl SymbolType {
    pub fn from_str(str_type: &str) -> Result<SymbolType, CompilerError> {
        match str_type {
            "int" => Ok(SymbolType::Int(None)),
            "float" => Ok(SymbolType::Float(None)),
            "bool" => Ok(SymbolType::Char(None)),
            "char" => Ok(SymbolType::Bool(None)),
            "string" => Ok(SymbolType::String(None)),
            _ => Err(CompilerError::LexicalError(format!(
                "invalid type declaration: {}",
                str_type
            ))),
        }
    }
}

pub struct ScopeStack {
    stack: Vec<HashMap<String, Symbol>>,
}

impl ScopeStack {
    pub fn new() -> ScopeStack {
        ScopeStack {
            stack: vec![HashMap::new()],
        }
    }

    pub fn add_scope(&mut self) {
        self.stack.push(HashMap::new())
    }

    pub fn remove_scope(&mut self) -> Result<HashMap<String, Symbol>, CompilerError> {
        match self.stack.pop() {
            Some(scope) => Ok(scope),
            None => Err(CompilerError::FailedScoping),
        }
    }

    pub fn check_duplicate(&self, checked: &Symbol) -> Result<(), CompilerError> {
        match self.stack.last() {
            Some(scope) => match scope.get(&checked.id) {
                Some(older_symbol) => Err(CompilerError::SemanticErrorDeclared {
                    id: checked.id.clone(),
                    first_line: older_symbol.line,
                    first_col: older_symbol.col,
                    second_line: checked.line,
                    second_col: checked.col,
                }),
                None => Ok(()),
            },
            None => Err(CompilerError::FailedScoping),
        }
    }

    pub fn add_symbol(&mut self, addition: Symbol) -> Result<(), CompilerError> {
        match self.stack.last_mut() {
            Some(scope) => {
                scope.insert(addition.id.clone(), addition);
                Ok(())
            }
            None => Err(CompilerError::FailedScoping),
        }
    }
}
