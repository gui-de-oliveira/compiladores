use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    /*
    // Examples of usage:
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    */
    #[error("Sanity error: {0}")]
    SanityError(String),

    #[error("Reading input file failure.")]
    IoReadFailure(#[from] std::io::Error),

    #[error("Lexical error: {0}")]
    LexicalError(String),

    #[error("Semantic error: {0}")]
    SemanticError(String),

    #[error("Parsing errors: {0}")]
    ParsingErrors(String),

    #[error("Tree building error: {0}")]
    TreeBuildingError(String),

    #[error("Parser failed to evaluate expression")]
    EvalParserFailure,

    #[error("Error in scope, this should not happen")]
    FailedScoping,

    #[error("Usage of undeclared identifier: \"{id}\"\nOccurrence at line {line}, column {col}:\n{highlight}")]
    SemanticErrorUndeclared {
        id: String,
        line: usize,
        col: usize,
        highlight: String,
    },

    #[error("Same-scope identifier redeclaration: \"{id}\"\nFirst occurrence at line {first_line}, column {first_col}:\n{first_highlight}\nAnd again at line {second_line}, column {second_col}:\n{second_highlight}")]
    SemanticErrorDeclared {
        id: String,
        first_line: usize,
        first_col: usize,
        first_highlight: String,
        second_line: usize,
        second_col: usize,
        second_highlight: String,
    },

    #[error("Variable identifier used as {second_class}: \"{id}\"\nFirst occurrence at line {first_line}, column {first_col}:\n{first_highlight}\nAnd again at line {second_line}, column {second_col}:\n{second_highlight}")]
    SemanticErrorVariable {
        id: String,
        first_line: usize,
        first_col: usize,
        first_highlight: String,
        second_class: String,
        second_line: usize,
        second_col: usize,
        second_highlight: String,
    },

    #[error("Vector identifier used as {second_class}: \"{id}\"\nFirst occurrence at line {first_line}, column {first_col}:\n{first_highlight}\nAnd again at line {second_line}, column {second_col}:\n{second_highlight}")]
    SemanticErrorVector {
        id: String,
        first_line: usize,
        first_col: usize,
        first_highlight: String,
        second_class: String,
        second_line: usize,
        second_col: usize,
        second_highlight: String,
    },

    #[error("Function identifier used as {second_class}: \"{id}\"\nFirst occurrence at line {first_line}, column {first_col}:\n{first_highlight}\nAnd again at line {second_line}, column {second_col}:\n{second_highlight}")]
    SemanticErrorFunction {
        id: String,
        first_line: usize,
        first_col: usize,
        first_highlight: String,
        second_class: String,
        second_line: usize,
        second_col: usize,
        second_highlight: String,
    },

    #[error("incompatible type in attribution")]
    SemanticErrorWrongType,

    #[error("Invalid type conversion from \"string\" to \"{invalid_type}\"\nOccurrence at line {line}, column {col}:\n{highlight}")]
    SemanticErrorStringToX {
        invalid_type: String,
        line: usize,
        col: usize,
        highlight: String,
    },

    #[error("Invalid type conversion from \"char\" to \"{invalid_type}\"\nOccurrence at line {line}, column {col}:\n{highlight}")]
    SemanticErrorCharToX {
        invalid_type: String,
        line: usize,
        col: usize,
        highlight: String,
    },

    #[error(
        "invalid attribution of type \"string\" value, size exceeds that of variable declaration"
    )]
    SemanticErrorStringMax,

    #[error("Invalid usage of \"string\" type for vector declaration: \"{id}\"\nOccurrence at line {line}, column {col}:\n{highlight}")]
    SemanticErrorStringVector {
        id: String,
        line: usize,
        col: usize,
        highlight: String,
    },

    #[error("missing args in function call \"{0}()\"")]
    SemanticErrorMissingArgs(String),

    #[error("excess args in function call \"{0}()\"")]
    SemanticErrorExcessArgs(String),

    #[error("invalid type in function call arguments")]
    SemanticErrorWrongTypeArgs,

    #[error("function argument or parameter of invalid type \"string\" ")]
    SemanticErrorFunctionString,

    #[error("Invalid argument for \"input\" command; expected variable of type \"int\" or \"float\", found \"{received_type}\";\nFirst occurrence at line {first_line}, column {first_col}:\n{first_highlight}\nAnd again at line {second_line}, column {second_col}:\n{second_highlight}")]
    SemanticErrorWrongParInput {
        received_type: String,
        first_highlight: String,
        first_line: usize,
        first_col: usize,
        second_highlight: String,
        second_line: usize,
        second_col: usize,
    },

    #[error("Invalid argument for \"output\" command; expected variable or literal of type \"int\" or \"float\", found \"{received_type}\";\nOccurrence at line {line}, column {col}:\n{highlight}")]
    SemanticErrorWrongParOutputLit {
        received_type: String,
        highlight: String,
        line: usize,
        col: usize,
    },

    #[error("Invalid argument for \"output\" command; expected variable or literal of type \"int\" or \"float\", found \"{received_type}\";\nFirst occurrence at line {first_line}, column {first_col}:\n{first_highlight}\nAnd again at line {second_line}, column {second_col}:\n{second_highlight}")]
    SemanticErrorWrongParOutputId {
        received_type: String,
        first_highlight: String,
        first_line: usize,
        first_col: usize,
        second_highlight: String,
        second_line: usize,
        second_col: usize,
    },

    #[error("invalid return for function; expected \"return\" command with compatible type")]
    SemanticErrorWrrongParReturn,

    #[error("invalid number parameter on shift command; expected number lower or equal to 16")]
    SemanticErrorWrongParShift,
}

impl CompilerError {
    pub fn error_code(&self) -> i32 {
        match *self {
            CompilerError::SanityError(_)
            | CompilerError::IoReadFailure(_)
            | CompilerError::LexicalError(_)
            | CompilerError::SemanticError(_)
            | CompilerError::ParsingErrors(_)
            | CompilerError::TreeBuildingError(_)
            | CompilerError::EvalParserFailure
            | CompilerError::FailedScoping => 1,
            CompilerError::SemanticErrorUndeclared { .. } => 10,
            CompilerError::SemanticErrorDeclared { .. } => 11,
            CompilerError::SemanticErrorVariable { .. } => 20,
            CompilerError::SemanticErrorVector { .. } => 21,
            CompilerError::SemanticErrorFunction { .. } => 22,
            CompilerError::SemanticErrorWrongType => 30,
            CompilerError::SemanticErrorStringToX { .. } => 31,
            CompilerError::SemanticErrorCharToX { .. } => 32,
            CompilerError::SemanticErrorStringMax => 33,
            CompilerError::SemanticErrorStringVector { .. } => 34,
            CompilerError::SemanticErrorMissingArgs(_) => 40,
            CompilerError::SemanticErrorExcessArgs(_) => 41,
            CompilerError::SemanticErrorWrongTypeArgs => 42,
            CompilerError::SemanticErrorFunctionString => 43,
            CompilerError::SemanticErrorWrongParInput { .. } => 50,
            CompilerError::SemanticErrorWrongParOutputLit { .. }
            | CompilerError::SemanticErrorWrongParOutputId { .. } => 51,
            CompilerError::SemanticErrorWrrongParReturn => 52,
            CompilerError::SemanticErrorWrongParShift => 53,
        }
    }
}
