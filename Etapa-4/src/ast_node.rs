use lrpar::NonStreamingLexer;
use lrpar::Span;
use std::ffi::c_void;
use std::fmt::Debug;

use super::error::CompilerError;
use super::semantic_structures::ScopeStack;

pub trait AstNode: Debug {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool);
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void);
    fn is_tree_member(&self) -> bool;
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>);
    fn evaluate_node(
        &self,
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<(), CompilerError>;
    fn get_id(&self) -> Span;
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
        stack: &mut ScopeStack,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<(), CompilerError> {
        self.as_ref().evaluate_node(stack, lexer)
    }
    fn get_id(&self) -> Span {
        self.as_ref().get_id()
    }
}

impl AstNode for Span {
    fn print_dependencies(&self, _own_address: *const c_void, _ripple: bool) {}
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        println!("{:p} [label=\"{}\"];", own_address, lexer.span_str(*self))
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn append_to_next(&mut self, _new_last: Box<dyn AstNode>) {}
    //TO DO: remove Span from tree
    fn evaluate_node(
        &self,
        _stack: &mut ScopeStack,
        _lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<(), CompilerError> {
        Ok(())
    }
    fn get_id(&self) -> Span {
        self.clone()
    }
}
