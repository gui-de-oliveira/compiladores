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
    #[error("reading input file failure")]
    IoReadFailure(#[from] std::io::Error),

    #[error("lexical error: {0}")]
    LexicalError(String),

    #[error("parsing errors: {0}")]
    ParsingErrors(String),

    #[error("tree building error: {0}")]
    TreeBuildingError(String),

    #[error("parser failed to evaluate expression")]
    EvalParserFailure,

    #[error("error in scope, this should not happen")]
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

    #[error("invalid type conversion from \"string\" to \"{0}\"")]
    SemanticErrorStringToX(String),

    #[error("invalid type conversion from \"char\" to \"{0}\"")]
    SemanticErrorCharToX(String),

    #[error(
        "invalid attribution of type \"string\" value, size exceeds that of variable declaration"
    )]
    SemanticErrorStringMax,

    #[error("invalid usage of \"string\" for vector data type")]
    SemanticErrorStringVector,

    #[error("missing args in function call \"{0}()\"")]
    SemanticErrorMissingArgs(String),

    #[error("excess args in function call \"{0}()\"")]
    SemanticErrorExcessArgs(String),

    #[error("invalid type in function call arguments")]
    SemanticErrorWrongTypeArgs,

    #[error("function argument or parameter of invalid type \"string\" ")]
    SemanticErrorFunctionString,

    #[error(
        "invalid type for \"input\" command; expected identifier of type \"int\" or \"float\""
    )]
    SemanticErrorWrongParInput,

    #[error("invalid type for \"output\" command; expected identifier or literal, of type \"int\" or \"float\"")]
    SemanticErrorWrongParOutput,

    #[error("invalid return for function; expected \"return\" command with compatible type")]
    SemanticErrorWrrongParReturn,

    #[error("invalid number parameter on shift command; expected number lower or equal to 16")]
    SemanticErrorWrongParShift,
}

impl CompilerError {
    pub fn error_code(&self) -> i32 {
        match *self {
            CompilerError::IoReadFailure(_)
            | CompilerError::LexicalError(_)
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
            CompilerError::SemanticErrorStringToX(_) => 31,
            CompilerError::SemanticErrorCharToX(_) => 32,
            CompilerError::SemanticErrorStringMax => 33,
            CompilerError::SemanticErrorStringVector => 34,
            CompilerError::SemanticErrorMissingArgs(_) => 40,
            CompilerError::SemanticErrorExcessArgs(_) => 41,
            CompilerError::SemanticErrorWrongTypeArgs => 42,
            CompilerError::SemanticErrorFunctionString => 43,
            CompilerError::SemanticErrorWrongParInput => 50,
            CompilerError::SemanticErrorWrongParOutput => 51,
            CompilerError::SemanticErrorWrrongParReturn => 52,
            CompilerError::SemanticErrorWrongParShift => 53,
        }
    }
}
