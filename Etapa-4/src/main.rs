// Grupo L
// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

mod abstract_syntax_tree;
mod ast_node;
mod auxiliary_lexical_structures;
mod error;
mod lexical_structures;
mod syntactic_structures;

use std::io::{self, Read, Write};

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

use error::CompilerError;

// Using `lrlex_mod!` brings the lexer for `scanner.l` into scope.
lrlex_mod!("scanner.l");
// Using `lrpar_mod!` brings the lexer for `parser.y` into scope.
lrpar_mod!("parser.y");

fn run_app() -> Result<(), CompilerError> {
    // We need to get a `LexerDef` for the `calc` language in order that we can lex input.
    let lexerdef = scanner_l::lexerdef();
    let stdin = io::stdin();
    let mut buffer = String::new();
    io::stdout().flush().ok();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    let lexer = lexerdef.lexer(&buffer);
    let (parsed, errors) = parser_y::parse(&lexer);
    for error in errors {
        println!("{}", error.pp(&lexer, &parser_y::token_epp));
    }

    match parsed {
        Some(Ok(abstract_syntax_tree)) => {
            abstract_syntax_tree.print_tree(&lexer);

            abstract_syntax_tree.evaluate(&lexer)?;
        }
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
            return Err(CompilerError::ParsingError);
        }
        None => {
            println!("Error: Unable to evaluate expression.");
            println!(">>> Failed input start!!");
            println!("{}", buffer);
            println!(">>> Failed input end!!");
            println!(">>> Debug start!!");
            println!("{:?}", buffer);
            println!(">>> Debug end!!");
            return Err(CompilerError::EvalParserFailure);
        }
    };
    Ok(())
}

fn app_entry_point() -> i32 {
    match run_app() {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("{:?}", error);
            error.error_code()
        }
    }
}

fn main() {
    std::process::exit(app_entry_point())
}
