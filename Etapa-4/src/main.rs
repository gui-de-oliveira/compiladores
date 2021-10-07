// Grupo L
// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

mod lexical_structures;
mod auxiliary_structures;
mod ast_node;

//use std::io::{self, BufRead, Write};
use std::io::{self, Read, Write};
use std::ptr::addr_of;
use std::ffi::c_void;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

use ast_node::AstNode;

// Using `lrlex_mod!` brings the lexer for `scanner.l` into scope.
lrlex_mod!("scanner.l");
// Using `lrpar_mod!` brings the lexer for `parser.y` into scope.
lrpar_mod!("parser.y");

fn main() {
    // We need to get a `LexerDef` for the `calc` language in order that we can lex input.
    let lexerdef = scanner_l::lexerdef();
    let stdin = io::stdin();
    io::stdout().flush().ok();
    let mut buffer = String::new();
    let mut handle = stdin.lock();

    match handle.read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(_) => (),
    };
    let lexer = lexerdef.lexer(&buffer);
    let (parsed, errors) = parser_y::parse(&lexer);
    for error in errors {
        println!("{}", error.pp(&lexer, &parser_y::token_epp));
    }

    if let Some(Ok(tree)) = parsed {
        let mut last_node = None;
        for top_level_def in &tree {
            if !top_level_def.is_tree_member() {
                continue
            }
            let current_node = addr_of!(*top_level_def) as *const c_void;
            if let Some(pointer) = last_node {
                println!("{:p}, {:p}", pointer, current_node);
            }
            top_level_def.print_dependencies(current_node);
            last_node = Some(current_node);
        }
        for top_level_def in &tree {
            if !top_level_def.is_tree_member() {
                continue
            }
            let current_node = addr_of!(*top_level_def) as *const c_void;
            top_level_def.print_labels(&lexer, current_node)
        }
    } else {
        println!("Unable to evaluate expression: {:?}", buffer);
    }
}
