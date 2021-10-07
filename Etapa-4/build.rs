// Grupo L
// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

use cfgrammar::yacc::YaccKind;
use lrlex::LexerBuilder;
use lrpar::{CTParserBuilder, RecoveryKind};

//Box<dyn std::error::Error>
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First we create the parser, which returns a HashMap of all the tokens used, then we pass
    // that HashMap to the lexer.
    //
    // Note that we specify the integer type (u8) we'll use for token IDs (this type *must* be big
    // enough to fit all IDs in) as well as the input file (which must end in ".y" for lrpar, and
    // ".l" for lrlex).
    let lex_rule_ids_map = CTParserBuilder::new()
        .yacckind(YaccKind::Grmtools)
        .recoverer(RecoveryKind::None) // Remove to enable the error recovery feature.
        .process_file_in_src("parser.y")?;
    LexerBuilder::new()
        .rule_ids_map(lex_rule_ids_map)
        .process_file_in_src("scanner.l")?;
    Ok(())
}
