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

program -> Result<AbstractSyntaxTree>:
     { /* %empty */ Ok(AbstractSyntaxTree::new(None)) }
    | topLevelDefList { Ok(AbstractSyntaxTree::new(Some($1?))) }
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
                    if var_or_vec.len() > 0 {
                        upper_def.append_to_next(top_level_def_assembler(is_static, var_type, var_or_vec)?);
                    }
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
        if name_def_vec.len() < 1 {
            bail!("localNameDefList returned vector with zero elements")
        };
        let mut top_name_def = mount_local_def(is_static, is_const, var_type, name_def_vec.remove(0));
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

literal -> Result<Box<dyn AstNode>>:
    literal_int { Ok(Box::new(LiteralInt::new($1?, None))) }
    | 'TK_LIT_FLOAT' { Ok(Box::new(LiteralFloat::new($span, None))) }
    | 'TK_LIT_FALSE' { Ok(Box::new(LiteralBool::new($span, None))) }
    | 'TK_LIT_TRUE' { Ok(Box::new(LiteralBool::new($span, None))) }
    | 'TK_LIT_CHAR' { Ok(Box::new(LiteralChar::new($span, None))) }
    | 'TK_LIT_STRING' { Ok(Box::new(LiteralString::new($span, None))) }
    ;

literal_int -> Result<Span>:
    'TK_LIT_INT' { Ok($span) }
    ;


simpleCommand -> Result<Box<dyn AstNode>>:
    varShift ';' { $1 }
    | varSet ';' { $1 }
    | IO ';' { $1 }
    | continueTok ';' { Ok(Box::new(Continue::new($1?, None))) }
    | breakTok ';' { Ok(Box::new(Break::new($1?, None))) }
    | returnTok expression ';' { Ok(Box::new(Return::new($1?, Box::new($2?), None))) }
    | functionCall ';' { $1 }
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
    identifier_rule leftShiftTok literal_int {
        let var_name = Box::new($1?);
        let shift_type = $2?;
        let shift_amount = Box::new($3?);
        Ok(Box::new(VarLeftShift::new(shift_type, var_name, shift_amount, None)))
    }
    | identifier_rule rightShiftTok literal_int {
        let var_name = Box::new($1?);
        let shift_type = $2?;
        let shift_amount = Box::new($3?);
        Ok(Box::new(VarRightShift::new(shift_type, var_name, shift_amount, None)))
    }
    | vecAccess leftShiftTok literal_int {
        let vec_access = $1?;
        let shift_type = $2?;
        let shift_amount = Box::new($3?);
        Ok(Box::new(VecLeftShift::new(shift_type, vec_access, shift_amount, None)))
    }
    | vecAccess rightShiftTok literal_int {
        let vec_access = $1?;
        let shift_type = $2?;
        let shift_amount = Box::new($3?);
        Ok(Box::new(VecRightShift::new(shift_type, vec_access, shift_amount, None)))
    }
    ;

vecAccess -> Result<Box<dyn AstNode>>:
    identifier_rule '[' expression ']' {
        let expr_span = $span;
        let vec_name = Box::new($1?);
        let vec_index = Box::new($3?);
        Ok(Box::new(VecAccess::new(expr_span, vec_name, vec_index, None)))
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
        let vec_access = $1?;
        let op_name = $2?;
        let new_value = Box::new($3?);
        Ok(Box::new(VecSet::new(op_name, vec_access, new_value, None)))
    }
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
        let lit_value = $2?;
        Ok(Box::new(OutputLit::new(op_name, lit_value, None)))
    }
    ;

inputTok -> Result<Span>:
    'TK_PR_INPUT' { Ok($span) }
    ;

outputTok -> Result<Span>:
    'TK_PR_OUTPUT' { Ok($span) }
    ;


functionCall -> Result<Box<dyn AstNode>>:
    identifier_rule '(' optionalExpressionList ')' {
        let fn_name = $1?;
        let args = $3?;
        Ok(Box::new(FnCall::new(fn_name, args, None)))
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


expression -> Result<Box<dyn AstNode>>:
    ternaryOrUniBooleanOrLower { $1 }
    ;

ternaryOrUniBooleanOrLower -> Result<Box<dyn AstNode>>:
    logicalOrOrLower questionTok ternaryOrUniBooleanOrLower doubleDotTok ternaryOrUniBooleanOrLower {
        let left_span = $2?;
        let right_span = $4?;
        let condition = Box::new($1?);
        let if_true = Box::new($3?);
        let if_false = Box::new($5?);
        Ok(Box::new(Ternary::new(left_span, right_span, condition, if_true, if_false, None)))
    }
    | logicalOrOrLower { $1 }
    ;

logicalOrOrLower -> Result<Box<dyn AstNode>>:
    logicalOrOrLower orTok logicalAndOrLower {
        let op_type = BinaryType::BoolOr;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | logicalAndOrLower { $1 }
    ;

logicalAndOrLower -> Result<Box<dyn AstNode>>:
    logicalAndOrLower andTok bitwiseOrOrLower {
        let op_type = BinaryType::BoolAnd;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | bitwiseOrOrLower { $1 }
    ;

bitwiseOrOrLower -> Result<Box<dyn AstNode>>:
    bitwiseOrOrLower pipeTok bitwiseXorOrLower {
        let op_type = BinaryType::BitOr;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | bitwiseXorOrLower { $1 }
    ;

bitwiseXorOrLower -> Result<Box<dyn AstNode>>:
    bitwiseXorOrLower circumflexTok bitwiseAndOrLower {
        let op_type = BinaryType::BitXor;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | bitwiseAndOrLower { $1 }
    ;

bitwiseAndOrLower -> Result<Box<dyn AstNode>>:
    bitwiseAndOrLower ampersandTok relationalEqualityOrLower {
        let op_type = BinaryType::BitAnd;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | relationalEqualityOrLower { $1 }
    ;

relationalEqualityOrLower -> Result<Box<dyn AstNode>>:
    relationalEqualityOrLower equalTok relationalSizeOrLower {
        let op_type = BinaryType::Equal;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | relationalEqualityOrLower notEqualTok relationalSizeOrLower {
        let op_type = BinaryType::NotEqual;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | relationalSizeOrLower { $1 }
    ;

relationalSizeOrLower -> Result<Box<dyn AstNode>>:
    relationalSizeOrLower lesserTok addSubOrLower {
        let op_type = BinaryType::Lesser;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | relationalSizeOrLower greaterTok addSubOrLower {
        let op_type = BinaryType::Greater;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | relationalSizeOrLower lesserEqualTok addSubOrLower {
        let op_type = BinaryType::LesserEqual;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | relationalSizeOrLower greaterEqualTok addSubOrLower {
        let op_type = BinaryType::GreaterEqual;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | addSubOrLower { $1 }
    ;

addSubOrLower -> Result<Box<dyn AstNode>>:
    addSubOrLower plusTok multDivRemainderOrLower {
        let op_type = BinaryType::Add;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | addSubOrLower minusTok multDivRemainderOrLower {
        let op_type = BinaryType::Sub;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | multDivRemainderOrLower { $1 }
    ;

multDivRemainderOrLower -> Result<Box<dyn AstNode>>:
    multDivRemainderOrLower multTok unaryOperationOrOperand {
        let op_type = BinaryType::Mult;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | multDivRemainderOrLower divTok unaryOperationOrOperand {
        let op_type = BinaryType::Div;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | multDivRemainderOrLower modTok unaryOperationOrOperand {
        let op_type = BinaryType::Mod;
        let op_span = $2?;
        let lhs = Box::new($1?);
        let rhs = Box::new($3?);
        Ok(Box::new(Binary::new(op_span, op_type, lhs, rhs, None)))
    }
    | unaryOperationOrOperand { $1 }
    ;

unaryOperationOrOperand -> Result<Box<dyn AstNode>>:
    expressionOperand { $1 }
    | unaryOperatorList expressionOperand {
        let expr = Box::new($2?);
        let mut op_list = $1?;
        if op_list.len() < 1 {
            bail!("unaryOperatorList returned vector with zero elements")
        };
        let (last_span, last_type) = op_list.pop().unwrap();
        let mut unary_box: Box<Unary> = Box::new(Unary::new(last_span, last_type, expr, None));
        loop {
            match op_list.pop() {
                Some((next_span, next_type)) => {
                    unary_box = Box::new(Unary::new(next_span, next_type, unary_box, None));
                },
                None => { break }
            }
        }
        let unary_node: Box<dyn AstNode> = unary_box;
        Ok(unary_node)
    }
    ;

unaryOperatorList -> Result<Vec<(Span, UnaryType)>>:
    unaryOperator { Ok(vec![$1?]) }
    | unaryOperatorList unaryOperator {
        let mut list = $1?;
        list.push($2?);
        Ok(list)
    }
    ;

unaryOperator -> Result<(Span, UnaryType)>:
    plusTok {
        let op_span = $span;
        let unary_type = UnaryType::Positive;
        Ok((op_span, unary_type))
        }
    | minusTok {
        let op_span = $span;
        let unary_type = UnaryType::Negative;
        Ok((op_span, unary_type))
        }
    | exclamationTok {
        let op_span = $span;
        let unary_type = UnaryType::Not;
        Ok((op_span, unary_type))
        }
    | ampersandTok {
        let op_span = $span;
        let unary_type = UnaryType::Address;
        Ok((op_span, unary_type))
        }
    | multTok {
        let op_span = $span;
        let unary_type = UnaryType::Pointer;
        Ok((op_span, unary_type))
        }
    | questionTok {
        let op_span = $span;
        let unary_type = UnaryType::Boolean;
        Ok((op_span, unary_type))
        }
    | hashTok {
        let op_span = $span;
        let unary_type = UnaryType::Hash;
        Ok((op_span, unary_type))
        }
    ;

expressionOperand -> Result<Box<dyn AstNode>>:
    literal { $1 }
    | accessOrFnCall { $1 }
    | grouping { $1 }
    ;

accessOrFnCall -> Result<Box<dyn AstNode>>:
    identifier_rule { Ok(Box::new(VarAccess::new($1?, None))) }
    | vecAccess { $1 }
    | functionCall { $1 }
    ;

grouping -> Result<Box<dyn AstNode>>:
    '(' expression ')' { $2 }
    ;

setTok -> Result<Span>:
    '=' { Ok($span) }
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

orTok -> Result<Span>:
    'TK_OC_OR' { Ok($span) }
    ;

andTok -> Result<Span>:
    'TK_OC_AND' { Ok($span) }
    ;

leftShiftTok -> Result<Span>:
    'TK_OC_SL' { Ok($span) }
    ;

rightShiftTok -> Result<Span>:
    'TK_OC_SR' { Ok($span) }
    ;

lesserTok -> Result<Span>:
    '<' { Ok($span) }
    ;

lesserEqualTok -> Result<Span>:
    'TK_OC_LE' { Ok($span) }
    ;

equalTok -> Result<Span>:
    'TK_OC_EQ' { Ok( $span ) }
    ;

notEqualTok -> Result<Span>:
    'TK_OC_NE' { Ok( $span ) }
    ;

greaterTok -> Result<Span>:
    '>' { Ok( $span ) }
    ;

greaterEqualTok -> Result<Span>:
    'TK_OC_GE' { Ok( $span ) }
    ;

doubleDotTok -> Result<Span>:
    ':' { Ok( $span ) }
    ;

minusTok -> Result<Span>:
    '-' { Ok( $span ) }
    ;

plusTok -> Result<Span>:
    '+' { Ok( $span ) }
    ;

divTok -> Result<Span>:
    '/' { Ok( $span ) }
    ;

multTok -> Result<Span>:
    '*' { Ok( $span ) }
    ;

modTok -> Result<Span>:
    '%' { Ok( $span ) }
    ;

circumflexTok -> Result<Span>:
    '^' { Ok( $span ) }
    ;

pipeTok -> Result<Span>:
    '|' { Ok( $span ) }
    ;

ampersandTok -> Result<Span>:
    '&' { Ok( $span ) }
    ;

exclamationTok -> Result<Span>:
    '!' { Ok( $span ) }
    ;

questionTok -> Result<Span>:
    '?' { Ok( $span ) }
    ;

hashTok -> Result<Span>:
    '#' { Ok( $span ) }
    ;

%%

use anyhow::{bail, Result};
use lrpar::Span;
use super::lexical_structures::*;
use super::auxiliary_structures::*;
use super::ast_node::AstNode;
use super::abstract_syntax_tree::AbstractSyntaxTree;


/*
void exporta(void* arvore) {
    printDependencies((ValorLexico *) arvore);
    printLabels((ValorLexico *) arvore);
}

void yyerror(char const *s) {
    printf("%s na linha %d\n", s, yylineno);
}
*/
