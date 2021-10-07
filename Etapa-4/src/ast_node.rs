use lrpar::NonStreamingLexer;
use std::ffi::c_void;

pub trait AstNode {
    fn print_dependencies(&self, own_address: *const c_void);
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void);
    fn is_tree_member(&self) -> bool;
}
