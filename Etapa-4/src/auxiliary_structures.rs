use super::lexical_structures::{Literal, Parameter, SimpleCommand};
use lrpar::Span;

#[derive(Debug)]
pub enum AuxVarOrVecName {
    Var(Span),
    Vec { name: Span, size: Span },
}

#[derive(Debug)]
pub enum AuxLocalNameDef {
    Def(Span),
    InitWithVar { var_name: Span, var_value: Span },
    InitWithLit { var_name: Span, var_value: Literal },
}

#[derive(Debug)]
pub enum AuxTopDefEnd {
    FnDefEnd {
        params: Vec<Parameter>,
        commands: Vec<SimpleCommand>,
    },
    SingleGlob,
    GlobList(Vec<AuxVarOrVecName>),
    VecAndGlobList(Span, Vec<AuxVarOrVecName>),
}
