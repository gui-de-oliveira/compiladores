use super::ast_node::AstNode;
use super::lexical_structures::{GlobalVarDef, GlobalVecDef, Literal, Parameter, SimpleCommand};
use anyhow::{bail, Result};
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

pub fn top_level_def_assembler(
    is_static: bool,
    var_type: Span,
    mut var_or_vec: Vec<AuxVarOrVecName>,
) -> Result<Box<dyn AstNode>> {
    var_or_vec.reverse();

    let mut last_step: Option<Box<dyn AstNode>> = None;
    match loop {
        let next_step = match var_or_vec.pop() {
            Some(def) => def,
            None => break last_step,
        };
        last_step = {
            let last_node = match last_step {
                Some(node) => Some(node as Box<dyn AstNode>),
                None => None,
            };
            Some(match next_step {
                AuxVarOrVecName::Var(var_name) => {
                    Box::new(GlobalVarDef::new(is_static, var_type, var_name, last_node))
                }
                AuxVarOrVecName::Vec { name, size } => Box::new(GlobalVecDef::new(
                    is_static, var_type, name, size, last_node,
                )),
            })
        }
    } {
        None => bail!("top_level_def_assembler() with empty length var_or_vec"),
        Some(def) => Ok(def),
    }
}
