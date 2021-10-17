// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

use lrpar::Span;

use crate::lexical_structures::LiteralInt;

use super::ast_node::AstNode;
use super::error::CompilerError;
use super::lexical_structures::{
    CommandBlock, GlobalVarDef, GlobalVecDef, LocalVarDef, Parameter, VarDefInitId, VarDefInitLit,
    VarInvoke,
};

#[derive(Debug)]
pub enum AuxVarOrVecName {
    Var(Span),
    Vec { name: Span, size: Box<LiteralInt> },
}

#[derive(Debug)]
pub enum AuxLocalNameDef {
    Def(Span),
    InitWithVar {
        var_name: Span,
        op_name: Span,
        var_value: Span,
    },
    InitWithLit {
        var_name: Span,
        op_name: Span,
        var_value: Box<dyn AstNode>,
    },
}

#[derive(Debug)]
pub enum AuxTopDefEnd {
    FnDefEnd {
        params: Vec<Parameter>,
        commands: CommandBlock,
    },
    SingleGlob,
    GlobList(Vec<AuxVarOrVecName>),
    VecAndGlobList(Box<LiteralInt>, Vec<AuxVarOrVecName>),
}

pub fn top_level_def_assembler(
    is_static: bool,
    var_type: Span,
    mut var_or_vec: Vec<AuxVarOrVecName>,
) -> Result<Box<dyn AstNode>, CompilerError> {
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
        None => {
            return Err(CompilerError::TreeBuildingError(
                "top_level_def_assembler() with empty length var_or_vec".to_string(),
            ))
        }
        Some(def) => Ok(def),
    }
}

pub fn mount_local_def(
    is_static: bool,
    is_const: bool,
    var_type: Span,
    name_def: AuxLocalNameDef,
) -> Box<dyn AstNode> {
    match name_def {
        AuxLocalNameDef::Def(var_name) => Box::new(LocalVarDef::new(
            is_static, is_const, var_type, var_name, false, None,
        )),
        AuxLocalNameDef::InitWithVar {
            var_name,
            op_name,
            var_value,
        } => Box::new(VarDefInitId::new(
            op_name,
            Box::new(LocalVarDef::new(
                is_static, is_const, var_type, var_name, true, None,
            )),
            Box::new(VarInvoke::new(var_value, None)),
            None,
        )),
        AuxLocalNameDef::InitWithLit {
            var_name,
            op_name,
            var_value,
        } => Box::new(VarDefInitLit::new(
            op_name,
            Box::new(LocalVarDef::new(
                is_static, is_const, var_type, var_name, true, None,
            )),
            var_value,
            None,
        )),
    }
}
