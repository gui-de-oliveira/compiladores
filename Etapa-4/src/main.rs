// Grupo L
// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

mod ast_node;
mod auxiliary_structures;
mod lexical_structures;

//use std::io::{self, BufRead, Write};
use std::ffi::c_void;
use std::io::{self, Read, Write};
use std::ptr::addr_of;

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
    let mut buffer = String::new();
    io::stdout().flush().ok();
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

    match parsed {
        Some(Ok(maybe_top_node)) => {
        let top_node: Box<dyn AstNode> = match maybe_top_node {
            Some(node) => node,
            None => return,
        };
        let address = addr_of!(*top_node) as *const c_void;
        top_node.print_dependencies(address, false);
        top_node.print_labels(&lexer, address);
        },
        Some(Err(error)) => {
            println!("Error: Unable to evaluate expression.");
            println!(">>> Failed input start!!");
            println!("{}", buffer);
            println!(">>> Failed input end!!");
            println!(">>> Debug start!!");
            println!("{:?}", buffer);
            println!(">>> Debug end!!");
            println!(">>> Error message start!!");
            println!("{:?}", error);
            println!(">>> Error message end!!");
        }
        None => {
            println!("Error: Unable to evaluate expression.");
            println!(">>> Failed input start!!");
            println!("{}", buffer);
            println!(">>> Failed input end!!");
            println!(">>> Debug start!!");
            println!("{:?}", buffer);
            println!(">>> Debug end!!");
        },
    }
}
