// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

use lrpar::NonStreamingLexer;
use lrpar::Span;
use std::ffi::c_void;
use std::fmt::Debug;

use super::error::CompilerError;
use super::instructions::IlocCode;
use super::semantic_structures::{ScopeStack, SymbolType};

pub trait AstNode: Debug {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool);
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void);
    fn is_tree_member(&self) -> bool;
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>);
    fn evaluate_node(
        &self,
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError>;
    fn get_id(&self) -> Span;
    fn get_next(&self) -> &Option<Box<dyn AstNode>>;
}

impl AstNode for Box<dyn AstNode> {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool) {
        self.as_ref().print_dependencies(own_address, ripple)
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        self.as_ref().print_labels(lexer, own_address)
    }
    fn is_tree_member(&self) -> bool {
        self.as_ref().is_tree_member()
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.as_mut().append_to_next(new_last)
    }
    fn evaluate_node(
        &self,
        code: &mut IlocCode,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<Option<SymbolType>, CompilerError> {
        self.as_ref().evaluate_node(code, stack, lexer)
    }
    fn get_id(&self) -> Span {
        self.as_ref().get_id()
    }
    fn get_next(&self) -> &Option<Box<dyn AstNode>> {
        self.as_ref().get_next()
    }
}
