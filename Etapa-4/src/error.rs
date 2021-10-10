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

    #[error("failure to parse input")]
    ParsingError,

    #[error("failure to evaluate expression")]
    EvalFailure,

    #[error("usage of undeclared identifier")]
    SemanticErrorUndeclared,

    #[error("same-scope identifier redeclaration")]
    SemanticErrorDeclared,

    #[error("variable identifier used as vector or function")]
    SemanticErrorVariable,

    #[error("vector identifier used as a variable or function")]
    SemanticErrorVector,

    #[error("function identifier used as a variable or vector")]
    SemanticErrorFunction,

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
            | CompilerError::ParsingError
            | CompilerError::EvalFailure => 1,
            CompilerError::SemanticErrorUndeclared => 10,
            CompilerError::SemanticErrorDeclared => 11,
            CompilerError::SemanticErrorVariable => 20,
            CompilerError::SemanticErrorVector => 21,
            CompilerError::SemanticErrorFunction => 22,
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
