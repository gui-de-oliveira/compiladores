use std::ffi::c_void;
use std::ptr::addr_of;

use lrpar::NonStreamingLexer;

use super::ast_node::AstNode;
use super::error::CompilerError;

pub struct AbstractSyntaxTree {
    top_node: Option<Box<dyn AstNode>>,
}

impl AbstractSyntaxTree {
    pub fn new(top_node: Option<Box<dyn AstNode>>) -> AbstractSyntaxTree {
        AbstractSyntaxTree { top_node }
    }

    pub fn print_tree(&self, lexer: &dyn NonStreamingLexer<u32>) {
        if let Some(node) = &self.top_node {
            let address = addr_of!(node) as *const c_void;
            node.print_dependencies(address, false);
            node.print_labels(lexer, address);
        }
    }

    pub fn evaluate(&self) -> Result<(), CompilerError> {
        Ok(())
    }
}
