// Grupo L
// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

mod abstract_syntax_tree;
mod ast_node;
mod auxiliary_lexical_structures;
mod error;
mod lexical_structures;
mod semantic_structures;

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
    let (parsed, mut errors) = parser_y::parse(&lexer);

    if errors.len() > 0 {
        let first_error = errors.remove(0);
        let mut report = first_error.pp(&lexer, &parser_y::token_epp);
        for error in errors {
            report.push_str(&error.pp(&lexer, &parser_y::token_epp));
        }
        return Err(CompilerError::ParsingErrors(report));
    }

    match parsed {
        Some(Ok(abstract_syntax_tree)) => {
            abstract_syntax_tree.evaluate(&lexer)?;

            abstract_syntax_tree.print_tree_code(&lexer);
            // abstract_syntax_tree.print_tree(&lexer);
        }
        Some(Err(error)) => {
            return Err(error);
        }
        None => {
            return Err(CompilerError::EvalParserFailure);
        }
    };
    Ok(())
}

fn app_entry_point() -> i32 {
    match run_app() {
        Ok(()) => 0,
        Err(error) => {
            println!("{}", error);
            error.error_code()
        }
    }
}

fn main() {
    std::process::exit(app_entry_point())
}
