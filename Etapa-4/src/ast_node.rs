use lrpar::NonStreamingLexer;
use lrpar::Span;
use std::collections::HashMap;
use std::ffi::c_void;
use std::fmt::Debug;

use super::error::CompilerError;
use super::syntactic_structures::Symbol;

pub trait AstNode: Debug {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool);
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void);
    fn is_tree_member(&self) -> bool;
    fn set_next(&mut self, new_next: Box<dyn AstNode>);
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>);
    fn evaluate_node(
        &self,
        stack: &mut Vec<HashMap<String, Symbol>>,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<(), CompilerError>;
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
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.as_mut().set_next(new_next)
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.as_mut().append_to_next(new_last)
    }
    fn evaluate_node(
        &self,
        stack: &mut Vec<HashMap<String, Symbol>>,
        lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<(), CompilerError> {
        self.as_ref().evaluate_node(stack, lexer)
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
    fn set_next(&mut self, _new_next: Box<dyn AstNode>) {}
    fn append_to_next(&mut self, _new_last: Box<dyn AstNode>) {}
    //TO DO: remove Span from tree
    fn evaluate_node(
        &self,
        _stack: &mut Vec<HashMap<String, Symbol>>,
        _lexer: &dyn NonStreamingLexer<u32>,
    ) -> Result<(), CompilerError> {
        Ok(())
    }
}
