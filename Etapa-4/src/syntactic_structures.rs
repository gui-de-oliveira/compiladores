use std::collections::HashMap;

use lrpar::{NonStreamingLexer, Span};

use super::error::CompilerError;

#[derive(Debug)]
pub struct Symbol {
    pub id: String,
    pub span: Span,
    pub line: usize,
    pub col: usize,
    pub type_value: SymbolType,
}

impl Symbol {
    pub fn new(id: String, span: Span, line: usize, col: usize, type_value: SymbolType) -> Symbol {
        Symbol {
            id,
            span,
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

    pub fn check_duplicate(
        &self,
        checked: &Symbol,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<(), CompilerError> {
        match self.stack.last() {
            Some(scope) => match scope.get(&checked.id) {
                Some(older_symbol) => Err(CompilerError::SemanticErrorDeclared {
                    id: checked.id.clone(),
                    first_line: older_symbol.line,
                    first_col: older_symbol.col,
                    first_string: ScopeStack::form_string_highlight(older_symbol.span, lexer),
                    second_line: checked.line,
                    second_col: checked.col,
                    second_string: ScopeStack::form_string_highlight(checked.span, lexer),
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

    pub fn form_string_highlight(span: Span, lexer: &dyn NonStreamingLexer<u32>) -> String {
        let ((_start_line, start_column), (_end_line, end_column)) = lexer.line_col(span);
        let mut lines = lexer.span_lines_str(span).lines().peekable();

        let mut output = String::new();

        let first_line = lines.next().unwrap_or("");
        output.push_str(&first_line);
        output.push('\n');

        for i in 0..start_column - 1 {
            output.push(' ');
        }
        let end_of_first_line = match lines.peek() {
            Some(_) => first_line.len() + 1,
            None => end_column,
        };
        for i in start_column..end_of_first_line {
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
            for i in 0..end_of_next_line {
                output.push('^');
            }
        }

        output
    }
}
