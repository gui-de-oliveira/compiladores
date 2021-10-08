%expect 0

%start program

%left 'TK_OC_OR'
%left 'TK_OC_AND'
%left '|'
%left '^'
%left '&'
%left 'TK_OC_EQ' 'TK_OC_NE'
%left '<' '>' 'TK_OC_LE' 'TK_OC_GE'
%left '+' '-'
%left '*' '/' '%'
%left '#' '!'
%left '?' ':'
%left '(' ')'

%%

program -> Result<Option<Box<dyn AstNode>>>:
     { /* %empty */ Ok(None) }
    | topLevelDefList { Ok(Some($1?)) }
    ;

topLevelDefList -> Result<Box<dyn AstNode>>:
    topLevelDef { $1 }
    | topLevelDefList topLevelDef {
        let mut upper_def = $1?;
        upper_def.append_to_next($2?);
        Ok(upper_def)
    }
    ;

topLevelDef -> Result<Box<dyn AstNode>>:
    optionalStatic type_rule identifier_rule topDefEnd {
        let is_static = $1?;
        let var_type = $2?;
        let name = $3?;
        Ok(
            match $4? {
                AuxTopDefEnd::FnDefEnd{params, commands} => {
                    Box::new(FnDef::new(is_static, var_type, name, params, commands, None))
                },
                AuxTopDefEnd::SingleGlob => {
                    Box::new(GlobalVarDef::new(is_static, var_type, name, None))
                },
                AuxTopDefEnd::GlobList(var_or_vec) => {
                    top_level_def_assembler(is_static, var_type, var_or_vec)?
                },
                AuxTopDefEnd::VecAndGlobList(vec_size, var_or_vec) => {
                    let mut upper_def = GlobalVecDef::new(is_static, var_type, name, vec_size, None);
                    upper_def.append_to_next(top_level_def_assembler(is_static, var_type, var_or_vec)?);
                    Box::new(upper_def)
                },
            }
        )
    }
    ;

topDefEnd -> Result<AuxTopDefEnd>:
    '(' optionalParamList ')' commandBlock { Ok(AuxTopDefEnd::FnDefEnd{params: $2?, commands: $4?}) }
    | ';' { Ok(AuxTopDefEnd::SingleGlob) }
    | ',' globDefEndList ';' { Ok(AuxTopDefEnd::GlobList($2?)) }
    | '[' literal_int ']' endOrGlobDefEndList { Ok(AuxTopDefEnd::VecAndGlobList($2?, $4?)) }
    ;

endOrGlobDefEndList -> Result<Vec<AuxVarOrVecName>>:
    ';' { Ok(vec![]) }
    | ',' globDefEndList ';' { $2 }
    ;

globDefEndList -> Result<Vec<AuxVarOrVecName>>:
    varOrVecNameList { $1 }
    ;

varOrVecNameList -> Result<Vec<AuxVarOrVecName>>:
    varOrVecName { Ok(vec![$1?]) }
    | varOrVecNameList ',' varOrVecName {
        let mut list = $1?;
        list.push($3?);
        Ok(list)
    }
    ;

varOrVecName -> Result<AuxVarOrVecName>:
    identifier_rule optionalArray {
        Ok(
            match $2? {
                Some(size) => {
                    let name = $1?;
                    AuxVarOrVecName::Vec{name, size}
                },
                None => { 
                    let span = $1?;
                    AuxVarOrVecName::Var(span)
                },
            }
        )
    }
    ;

optionalArray -> Result<Option<Span>>:
      { /* %empty */ Ok(None) }
    | '[' literal_int ']' { Ok(Some($2?)) }
    ;

optionalStatic -> Result<bool>:
      { /* %empty */ Ok(false) }
    | 'TK_PR_STATIC' { Ok(true) }
    ;

type_rule -> Result<Span>:
    'TK_PR_INT' { Ok($span) }
    | 'TK_PR_FLOAT' { Ok($span) }
    | 'TK_PR_BOOL' { Ok($span) }
    | 'TK_PR_CHAR' { Ok($span) }
    | 'TK_PR_STRING' { Ok($span) }
    ;

identifier_rule -> Result<Span>:
    'TK_IDENTIFICADOR' { Ok($span) }
    ;

literal_int -> Result<Span>:
    'TK_LIT_INT' { Ok($span) }
    ;

optionalParamList -> Result<Vec<Parameter>>:
      { /* %empty */ Ok(vec![]) }
    | paramList { $1 }
    ;

paramList -> Result<Vec<Parameter>>:
    param { Ok(vec![$1?]) }
    | paramList ',' param {
        let mut list = $1?;
        list.push($3?);
        Ok(list)
    }
    ;

param -> Result<Parameter>:
    optionalConst type_rule identifier_rule {
        let is_const = $1?;
        let param_type = $2?;
        let param_name = $3?;
        Ok(Parameter{is_const, param_type, param_name})
    }
    ;

optionalConst -> Result<bool>:
     { /* %empty */ Ok(false) }
    | 'TK_PR_CONST' { Ok(true) }
    ;


commandBlock -> Result<Box<dyn AstNode>>:
    '{' optionalSimpleCommandList '}' { 
        match $2? {
            Some(node) => Ok(node),
            None => Ok(Box::new(EmptyBlock::new($span, None))),
        }
    }
    ;

optionalSimpleCommandList -> Result<Option<Box<dyn AstNode>>>:
      { /* %empty */ Ok(None) }
    | simpleCommandList { Ok(Some($1?)) }
    ;

simpleCommandList -> Result<Box<dyn AstNode>>:
    simpleCommandSequence { $1 }
    | simpleCommandList simpleCommandSequence {
        let mut left_node = $1?;
        let right_node = $2?;
        left_node.append_to_next(right_node);
        Ok(left_node)
    }
    ;

simpleCommandSequence -> Result<Box<dyn AstNode>>:
    commandBlock ';' { $1 }
    | localDefList ';' { $1 }
    | simpleCommand { $1 }
    ;

localDefList -> Result<Box<dyn AstNode>>:
    optionalStatic optionalConst type_rule localNameDefList {
        let is_static = $1?;
        let is_const = $2?;
        let var_type = $3?;
        let mut name_def_vec = $4?;
        let mut top_name_def = mount_local_def(is_static, is_const, var_type, name_def_vec.pop().unwrap());
        for name_def in name_def_vec {
            top_name_def.append_to_next(mount_local_def(is_static, is_const, var_type, name_def))
        };
        Ok(Box::new(top_name_def))
    }
    ;

localNameDefList -> Result<Vec<AuxLocalNameDef>>:
    identifier_rule { Ok(vec![AuxLocalNameDef::Def($1?)]) }
    | localNameDefAssign { Ok(vec![$1?]) }
    | localNameDefList ',' identifier_rule {
        let mut list = $1?;
        list.push(AuxLocalNameDef::Def($3?));
        Ok(list)
    }
    | localNameDefList ',' localNameDefAssign {
        let mut list = $1?;
        list.push($3?);
        Ok(list)
    }
    ;

localNameDefAssign -> Result<AuxLocalNameDef>:
    identifier_rule lesserEqualTok identifier_rule {
        let var_name = $1?;
        let op_name = $2?;
        let var_value = $3?;
        Ok(AuxLocalNameDef::InitWithVar{var_name, op_name, var_value})
    }
    | identifier_rule lesserEqualTok literal {
        let var_name = $1?;
        let op_name = $2?;
        let var_value = $3?;
        Ok(AuxLocalNameDef::InitWithLit{var_name, op_name, var_value})
    }
    ;

lesserEqualTok -> Result<Span>:
    'TK_OC_LE' { Ok($span) }
    ;

literal -> Result<Literal>:
    literal_int { Ok(Literal::Int($1?)) }
    | 'TK_LIT_FLOAT' { Ok(Literal::Float($span)) }
    | 'TK_LIT_FALSE' { Ok(Literal::Bool($span)) }
    | 'TK_LIT_TRUE' { Ok(Literal::Bool($span)) }
    | 'TK_LIT_CHAR' { Ok(Literal::Char($span)) }
    | 'TK_LIT_STRING' { Ok(Literal::String($span)) }
    ;


simpleCommand -> Result<Box<dyn AstNode>>:
    varShift ';' { $1 }
    | varSet ';' { $1 }
    | IO ';' { $1 }
    | continueTok ';' { Ok(Box::new(Continue::new($1?, None))) }
    | breakTok ';' { Ok(Box::new(Break::new($1?, None))) }
    | returnTok expression ';' { Ok(Box::new(Return::new($1?, Box::new($2?), None))) }
    | functionCall ';' { Ok(Box::new($1?)) }
    | conditional ';' { $1 }
    ;

continueTok -> Result<Span>:
    'TK_PR_CONTINUE' { Ok($span) }
    ;

breakTok -> Result<Span>:
    'TK_PR_BREAK' { Ok($span) }
    ;

returnTok -> Result<Span>:
    'TK_PR_RETURN' { Ok($span) }
    ;

varShift -> Result<Box<dyn AstNode>>:
    identifier_rule shiftOperator literal_int {
        let var_name = $1?;
        let shift_type = $2?;
        let shift_amount = $3?;
        Ok(Box::new(VarShift::new(shift_type, Box::new(var_name), Box::new(shift_amount), None)))
    }
    | vecAccess shiftOperator literal_int {
        let vec_access = $1?;
        let shift_type = $2?;
        let shift_amount = $3?;
        Ok(Box::new(VecShift::new(shift_type, Box::new(vec_access), Box::new(shift_amount), None)))
    }
    ;

shiftOperator -> Result<Span>:
    'TK_OC_SR' { Ok($span) }
    | 'TK_OC_SL' { Ok($span) }
    ;

vecAccess -> Result<VecAccess>:
    identifier_rule '[' expression ']' {
        let name = Box::new($1?);
        let index = Box::new($3?);
        Ok(VecAccess{name, index})
    }
    ;


varSet -> Result<Box<dyn AstNode>>:
    identifier_rule setTok expression {
        let var_name = Box::new($1?);
        let op_name = $2?;
        let new_value = Box::new($3?);
        Ok(Box::new(VarSet::new(op_name, var_name, new_value, None)))
    }
    | vecAccess setTok expression {
        let vec_access = Box::new($1?);
        let op_name = $2?;
        let new_value = Box::new($3?);
        Ok(Box::new(VecSet::new(op_name, vec_access, new_value, None)))
    }
    ;

setTok -> Result<Span>:
    '=' { Ok($span) }
    ;

IO -> Result<Box<dyn AstNode>>:
    inputTok identifier_rule {
        let op_name = $1?;
        let var_name = Box::new($2?);
        Ok(Box::new(Input::new(op_name, var_name, None)))
    }
    | outputTok identifier_rule {
        let op_name = $1?;
        let var_name = Box::new($2?);
        Ok(Box::new(OutputId::new(op_name, var_name, None)))
    }
    | outputTok literal {
        let op_name = $1?;
        let lit_value = Box::new($2?);
        Ok(Box::new(OutputLit::new(op_name, lit_value, None)))
    }
    ;

inputTok -> Result<Span>:
    'TK_PR_INPUT' { Ok($span) }
    ;

outputTok -> Result<Span>:
    'TK_PR_OUTPUT' { Ok($span) }
    ;


functionCall -> Result<FnCall>:
    identifier_rule '(' optionalExpressionList ')' {
        let fn_name = $1?;
        let args = $3?;
        Ok(FnCall::new(fn_name, args, None))
    }
    ;

optionalExpressionList -> Result<Option<Box<dyn AstNode>>>:
      { /* %empty */ Ok(None) }
    | expressionList { Ok(Some($1?)) } 
    ;

expressionList -> Result<Box<dyn AstNode>>:
    expression { Ok(Box::new($1?)) }
    | expressionList ',' expression {
        let mut expr = $1?;
        expr.append_to_next(Box::new($3?));
        Ok(expr)
    }
    ;


conditional -> Result<Box<dyn AstNode>>:
    ifTok '(' expression ')' commandBlock {
        let op_name = $1?;
        let condition = Box::new($3?);
        let consequence =  Box::new($5?);
        Ok(Box::new(If::new(op_name, condition, consequence, None)))
    }
    | ifTok '(' expression ')' commandBlock 'TK_PR_ELSE' commandBlock {
        let op_name = $1?;
        let condition = Box::new($3?);
        let if_true = Box::new($5?);
        let if_false = Box::new($7?);
        Ok(Box::new(IfElse::new(op_name, condition, if_true, if_false, None)))
    }
    | forTok '(' varSet ':' expression ':' varSet ')' commandBlock {
        let op_name = $1?;
        let count_init = Box::new($3?);
        let count_check = Box::new($5?);
        let count_iter = Box::new($7?);
        let actions = Box::new($9?);
        Ok(Box::new(For::new(op_name, count_init, count_check, count_iter, actions, None)))
    }
    | whileTok '(' expression ')' 'TK_PR_DO' commandBlock {
        let op_name = $1?;
        let condition = Box::new($3?);
        let consequence =  Box::new($6?);
        Ok(Box::new(While::new(op_name, condition, consequence, None)))
    }
    ;

ifTok -> Result<Span>:
    'TK_PR_IF' { Ok($span) }
    ;

forTok -> Result<Span>:
    'TK_PR_FOR' { Ok($span) }
    ;

whileTok -> Result<Span>:
    'TK_PR_WHILE' { Ok($span) }
    ;


expression -> Result<Expression>:
    ternaryOrUniBooleanOrLower { $1 }
    ;

ternaryOrUniBooleanOrLower -> Result<Expression>:
    logicalOrOrLower '?' ternaryOrUniBooleanOrLower ':' ternaryOrUniBooleanOrLower {
        let condition = Box::new($1?);
        let if_true = Box::new($3?);
        let if_false = Box::new($5?);
        Ok(Expression::Ternary{condition, if_true, if_false})
    }
    | logicalOrOrLower { $1 }
    ;

logicalOrOrLower -> Result<Expression>:
    logicalOrOrLower 'TK_OC_OR' logicalAndOrLower {
        let op_type = BinaryType::BoolOr;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | logicalAndOrLower { $1 }
    ;

logicalAndOrLower -> Result<Expression>:
    logicalAndOrLower 'TK_OC_AND' bitwiseOrOrLower {
        let op_type = BinaryType::BoolAnd;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | bitwiseOrOrLower { $1 }
    ;

bitwiseOrOrLower -> Result<Expression>:
    bitwiseOrOrLower '|' bitwiseXorOrLower {
        let op_type = BinaryType::BitOr;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | bitwiseXorOrLower { $1 }
    ;

bitwiseXorOrLower -> Result<Expression>:
    bitwiseXorOrLower '^' bitwiseAndOrLower {
        let op_type = BinaryType::BitXor;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | bitwiseAndOrLower { $1 }
    ;

bitwiseAndOrLower -> Result<Expression>:
    bitwiseAndOrLower '&' relationalEqualityOrLower {
        let op_type = BinaryType::BitAnd;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | relationalEqualityOrLower { $1 }
    ;

relationalEqualityOrLower -> Result<Expression>:
    relationalEqualityOrLower 'TK_OC_EQ' relationalSizeOrLower {
        let op_type = BinaryType::Equal;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | relationalEqualityOrLower 'TK_OC_NE' relationalSizeOrLower {
        let op_type = BinaryType::NotEqual;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | relationalSizeOrLower { $1 }
    ;

relationalSizeOrLower -> Result<Expression>:
    relationalSizeOrLower '<' addSubOrLower {
        let op_type = BinaryType::Lesser;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | relationalSizeOrLower '>' addSubOrLower {
        let op_type = BinaryType::Greater;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | relationalSizeOrLower lesserEqualTok addSubOrLower {
        let op_type = BinaryType::LesserEqual;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | relationalSizeOrLower 'TK_OC_GE' addSubOrLower {
        let op_type = BinaryType::GreaterEqual;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | addSubOrLower { $1 }
    ;

addSubOrLower -> Result<Expression>:
    addSubOrLower '+' multDivRemainderOrLower {
        let op_type = BinaryType::Add;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | addSubOrLower '-' multDivRemainderOrLower {
        let op_type = BinaryType::Sub;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | multDivRemainderOrLower { $1 }
    ;

multDivRemainderOrLower -> Result<Expression>:
    multDivRemainderOrLower '*' unaryOperationOrOperand {
        let op_type = BinaryType::Mult;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | multDivRemainderOrLower '/' unaryOperationOrOperand {
        let op_type = BinaryType::Div;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | multDivRemainderOrLower '%' unaryOperationOrOperand {
        let op_type = BinaryType::Mod;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Expression::Binary{op_type, lhs, rhs})
    }
    | unaryOperationOrOperand { $1 }
    ;

unaryOperationOrOperand -> Result<Expression>:
    expressionOperand { $1 }
    | unaryOperatorList expressionOperand {
        let mut op_list = $1?;
        op_list.reverse();
        let mut wrapping = $2?;
        loop {
            match op_list.pop() {
                Some(op_type) => {
                    wrapping = Expression::Unary{op_type, operand: Box::new(wrapping)};
                },
                None => { break }
            }
        }
        Ok(wrapping)
    }
    ;

unaryOperatorList -> Result<Vec<UnaryType>>:
    unaryOperator { Ok(vec![$1?]) }
    | unaryOperatorList unaryOperator {
        let mut list = $1?;
        list.push($2?);
        Ok(list)
    }
    ;

unaryOperator -> Result<UnaryType>:
    '+' { Ok(UnaryType::Positive) }
    | '-' { Ok(UnaryType::Negative) }
    | '!' { Ok(UnaryType::Not) }
    | '&' { Ok(UnaryType::Address) }
    | '*' { Ok(UnaryType::Pointer) }
    | '?' { Ok(UnaryType::Boolean) }
    | '#' { Ok(UnaryType::Hash) }
    ;

expressionOperand -> Result<Expression>:
    literal { Ok(Expression::Literal($1?)) }
    | accessOrFnCall { $1 }
    | grouping { $1 }
    ;

accessOrFnCall -> Result<Expression>:
    identifier_rule { Ok(Expression::VarAccess($1?)) }
    | vecAccess {
        let vec_access = $1?;
        Ok(Expression::VecAccess(vec_access))
    }
    | functionCall { Ok(Expression::FnCall($1?)) }
    ;

grouping -> Result<Expression>:
    '(' expression ')' { $2 }
    ;

%%

use anyhow::Result;
use lrpar::Span;
use super::lexical_structures::*;
use super::auxiliary_structures::*;
use super::ast_node::AstNode;


/*
void exporta(void* arvore) {
    printDependencies((ValorLexico *) arvore);
    printLabels((ValorLexico *) arvore);
}

void libera(void* arvore) {
    if(arvore == NULL) {
        return;
    }
    freeValorLexico((ValorLexico*) arvore);
}

void yyerror(char const *s) {
    printf("%s na linha %d\n", s, yylineno);
}
*/
