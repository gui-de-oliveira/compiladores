// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

use std::ffi::c_void;
use std::ptr::addr_of;

use lrpar::NonStreamingLexer;

use super::ast_node::AstNode;
use super::error::CompilerError;
use super::semantic_structures::ScopeStack;

pub struct AbstractSyntaxTree {
    top_node: Option<Box<dyn AstNode>>,
    code: Vec<String>,
}

impl AbstractSyntaxTree {
    pub fn new(top_node: Option<Box<dyn AstNode>>) -> AbstractSyntaxTree {
        let mut code: Vec<String> = Vec::new();

        // Start point of data segment, stack pointer, and frame pointer
        code.push("loadI 1024 => rfp".to_string());
        code.push("loadI 1024 => rsp".to_string());
        code.push("loadI 18 => rbss".to_string());

        code.push("loadI 8 => r3".to_string());
        code.push("storeAI r3 => rsp, 0".to_string());

        code.push("storeAI rsp => rsp, 4".to_string()); // saves rsp
        code.push("storeAI rfp => rsp, 8".to_string()); // saves rfp
        code.push("jumpI -> L0".to_string()); // jumps to main

        code.push("halt".to_string());

        // FAKE VALUE (SHOULD GET FROM MAIN)

        code.push("L0:".to_string()); // function label
        code.push("i2i rsp => rfp".to_string()); // set rfp to rsp
        code.push("addI rsp, 16 => rsp".to_string()); // updates rsp

        // should copy command block's code here ><

        code.push("loadAI rfp, 0 => r0".to_string()); // return address
        code.push("loadAI rfp, 4 => r1".to_string()); // restore rsp
        code.push("loadAI rfp, 8 => r2".to_string()); // restore rfp

        code.push("i2i r1 => rsp".to_string()); // idk
        code.push("i2i r2 => rfp".to_string()); // idk

        code.push("jump -> r0".to_string()); // returns

        AbstractSyntaxTree { top_node, code }
    }

    pub fn print_tree(&self, lexer: &dyn NonStreamingLexer<u32>) {
        for line in &self.code {
            println!("{}", line)
        }

        // TO DELETE:
        // if let Some(node) = &self.top_node {
        //     let address = addr_of!(node) as *const c_void;
        //     node.print_dependencies(address, false);
        //     node.print_labels(lexer, address);
        // }
    }

    pub fn evaluate(&self, lexer: &dyn NonStreamingLexer<u32>) -> Result<(), CompilerError> {
        if let Some(node) = &self.top_node {
            let mut stack = ScopeStack::new();
            node.evaluate_node(&mut stack, lexer)?;
        };
        Ok(())
    }
}
