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
    pub class: SymbolClass,
}

impl Symbol {
    pub fn new(
        id: String,
        span: Span,
        line: usize,
        col: usize,
        type_value: SymbolType,
        class: SymbolClass,
    ) -> Symbol {
        Symbol {
            id,
            span,
            line,
            col,
            type_value,
            class,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SymbolType {
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SymbolClass {
    Fn,
    Var,
    Vec,
    Lit,
}

impl SymbolClass {
    pub fn to_str(&self) -> &str {
        match self {
            SymbolClass::Fn => "function",
            SymbolClass::Var => "variable",
            SymbolClass::Vec => "vector",
            SymbolClass::Lit => "literal",
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
        span: Span,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<(), CompilerError> {
        match self.stack.last() {
            Some(scope) => {
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

    pub fn get_previous_def(
        &self,
        span: Span,
        lexer: &dyn NonStreamingLexer<u32>,
        expected_class: SymbolClass,
    ) -> Result<&Symbol, CompilerError> {
        let id = lexer.span_str(span).to_string();
        for scope in self.stack.iter().rev() {
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
                        SymbolClass::Var => CompilerError::SemanticErrorVariable {
                            id,
                            first_line,
                            first_col,
                            first_highlight,
                            second_class,
                            second_line,
                            second_col,
                            second_highlight,
                        },
                        SymbolClass::Vec => CompilerError::SemanticErrorVector {
                            id,
                            first_line,
                            first_col,
                            first_highlight,
                            second_class,
                            second_line,
                            second_col,
                            second_highlight,
                        },
                        SymbolClass::Fn => CompilerError::SemanticErrorFunction {
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
