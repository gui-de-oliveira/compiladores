use lrpar::NonStreamingLexer;
use std::ffi::c_void;
use std::fmt::Debug;

pub trait AstNode: Debug {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool);
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void);
    fn is_tree_member(&self) -> bool;
    fn set_next(&mut self, new_next: Box<dyn AstNode>);
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>);
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
}
