%{
    // Grupo L
    // Guilherme de Oliveira (00278301)
    // Jean Pierre Comerlatto Darricarrere (00182408)

    #include "abstract_syntax_tree.h"

    extern void *arvore;
    extern int yylineno;

    struct DummyFunctionDef DUMMY = { .label = "DUMMY"};

    void exporta(void*);
    int yylex(void);
    void yyerror (char const *s);
%}

%union {
    struct ValorLexico valor_lexico;
    struct CommandList* optional_command_list;
    struct FunctionDef* optional_function_def;
    struct DummyFunctionDef dummy_function_def;
}

%expect 0

%start program

%token<valor_lexico> TK_PR_INT
%token<valor_lexico> TK_PR_FLOAT
%token<valor_lexico> TK_PR_BOOL
%token<valor_lexico> TK_PR_CHAR
%token<valor_lexico> TK_PR_STRING
%token<valor_lexico> TK_PR_IF
%token<valor_lexico> TK_PR_THEN
%token<valor_lexico> TK_PR_ELSE
%token<valor_lexico> TK_PR_WHILE
%token<valor_lexico> TK_PR_DO
%token<valor_lexico> TK_PR_INPUT
%token<valor_lexico> TK_PR_OUTPUT
%token<valor_lexico> TK_PR_RETURN
%token<valor_lexico> TK_PR_CONST
%token<valor_lexico> TK_PR_STATIC
%token<valor_lexico> TK_PR_FOREACH
%token<valor_lexico> TK_PR_FOR
%token<valor_lexico> TK_PR_SWITCH
%token<valor_lexico> TK_PR_CASE
%token<valor_lexico> TK_PR_BREAK
%token<valor_lexico> TK_PR_CONTINUE
%token<valor_lexico> TK_PR_CLASS
%token<valor_lexico> TK_PR_PRIVATE
%token<valor_lexico> TK_PR_PUBLIC
%token<valor_lexico> TK_PR_PROTECTED
%token<valor_lexico> TK_PR_END
%token<valor_lexico> TK_PR_DEFAULT
%token<valor_lexico> TK_OC_LE
%token<valor_lexico> TK_OC_GE
%token<valor_lexico> TK_OC_EQ
%token<valor_lexico> TK_OC_NE
%token<valor_lexico> TK_OC_AND
%token<valor_lexico> TK_OC_OR
%token<valor_lexico> TK_OC_SL
%token<valor_lexico> TK_OC_SR
%token<valor_lexico> TK_LIT_INT
%token<valor_lexico> TK_LIT_FLOAT
%token<valor_lexico> TK_LIT_FALSE
%token<valor_lexico> TK_LIT_TRUE
%token<valor_lexico> TK_LIT_CHAR
%token<valor_lexico> TK_LIT_STRING
%token<valor_lexico> TK_IDENTIFICADOR
%token<valor_lexico> TOKEN_ERRO

%type<optional_command_list> commandBlock
%type<optional_function_def> program
%type<optional_function_def> topLevelDefList
%type<optional_function_def> functionDef

%left GROUPING
%right GROUPING_CLOSE

%nonassoc EXPRESSION_OPERAND
%left EXPRESSION

%left TERNARY_OPERATION

%right TERNARY_CLOSE
%left TERNARY_OPEN

%left BINARY_OPERATION

%left LOGICAL_OR
%left LOGICAL_AND
%left BITWISE_OR
%left BITWISE_XOR
%left BITWISE_AND
%left RELATIONAL_EQUALITY_OP
%left RELATIONAL_SIZE_OP
%left SHIFT_OPERATOR
%left ADD_SUB
%left MULT_DIV_REMAINDER

%left UNARY_OPERATOR


%%

program:
    %empty {
        $$ = NULL;
    }
    | topLevelDefList {
        $$ = $1;
        arvore = $$;
    }
    ;

topLevelDefList:
    globalDef {
        $$ = NULL;
    }
    | functionDef {
        $$ = $1;
    }
    | topLevelDefList globalDef {
        $$ = $1;
    }
    | topLevelDefList functionDef {
        append_function_def($1, $2);
        $$ = $1;
    }
    ;

functionDef:
    optionalStatic type TK_IDENTIFICADOR '(' optionalParamList ')' commandBlock {
        $$ = new_function_def($3, $7);
    }
    ;

optionalStatic:
    %empty
    | TK_PR_STATIC
    ;

type:
    TK_PR_INT
    | TK_PR_FLOAT
    | TK_PR_BOOL
    | TK_PR_CHAR
    | TK_PR_STRING
    ;

optionalArray:
    %empty
    | '[' TK_LIT_INT ']'
    ;

varOrVecName:
    TK_IDENTIFICADOR optionalArray
    ;

varOrVecNameList:
    varOrVecName
    | varOrVecNameList ',' varOrVecName
    ;

globalDef:
    optionalStatic type varOrVecNameList ';'
    ;

optionalConst:
    %empty
    | TK_PR_CONST
    ;

optionalParamList:
    %empty
    | paramList
    ;

paramList:
    param
    | paramList ',' param
    ;

param:
    optionalConst type TK_IDENTIFICADOR
    ;


commandBlock:
    '{' optionalSimpleCommandList '}' {
        $$ = NULL ; // TODO
    }
    ;

optionalSimpleCommandList:
    %empty
    | simpleCommandList
    ;

simpleCommandList:
    simpleCommand
    | simpleCommandList simpleCommand
    ;

simpleCommand:
    commandBlock ';'
    | localDef ';'
    | varSet ';'
    | varShift ';'
    | conditional ';'
    | IO ';'
    | functionCall ';'
    | TK_PR_RETURN expression ';'
    | TK_PR_CONTINUE ';'
    | TK_PR_BREAK ';'
    ;


literal:
    TK_LIT_INT
    | TK_LIT_FLOAT
    | TK_LIT_FALSE
    | TK_LIT_TRUE
    | TK_LIT_CHAR
    | TK_LIT_STRING
    ;


localDef:
    optionalStatic optionalConst type localNameDefList
    ;

localNameDefList:
    localNameDef
    | localNameDefList ',' localNameDef
    ;

localNameDef:
    TK_IDENTIFICADOR
    | TK_IDENTIFICADOR TK_OC_LE TK_IDENTIFICADOR
    | TK_IDENTIFICADOR TK_OC_LE literal
    ;


optionalArrayAccess:
    %empty
    | '[' expression ']'
    ;

varSet:
    TK_IDENTIFICADOR optionalArrayAccess '=' expression
    ;
    

varShift:
    TK_IDENTIFICADOR optionalArrayAccess shiftOperator expression
    ;

shiftOperator:
    TK_OC_SR %prec SHIFT_OPERATOR
    | TK_OC_SL %prec SHIFT_OPERATOR
    ;


functionCall:
    TK_IDENTIFICADOR '(' optionalExpressionList ')'
    ;


IO:
    TK_PR_INPUT TK_IDENTIFICADOR
    | TK_PR_OUTPUT TK_IDENTIFICADOR
    | TK_PR_OUTPUT literal
    ;


conditional:
    TK_PR_IF '(' expression ')' commandBlock
    | TK_PR_IF '(' expression ')' commandBlock TK_PR_ELSE commandBlock
    | TK_PR_FOR '(' varSet ':' expression ':' varSet ')' commandBlock
    | TK_PR_WHILE '(' expression ')' TK_PR_DO commandBlock
    ;

expression:
    ternaryOperationOrLower
    ;

ternaryOperationOrLower:
    binaryOperationOrLower ternaryOpen ternaryOperationOrLower ternaryClose ternaryOperationOrLower
    | binaryOperationOrLower
    ;

ternaryOpen:
    '?' %prec TERNARY_OPEN
    ;

ternaryClose:
    ':' %prec TERNARY_CLOSE
    ;

binaryOperationOrLower:
    binaryOperationOrLower logicalOr unaryOperationOrOperand %prec LOGICAL_OR
    | binaryOperationOrLower logicalAnd unaryOperationOrOperand %prec LOGICAL_AND
    | binaryOperationOrLower bitwiseOr unaryOperationOrOperand %prec BITWISE_OR
    | binaryOperationOrLower bitwiseXor unaryOperationOrOperand %prec BITWISE_XOR
    | binaryOperationOrLower bitwiseAnd unaryOperationOrOperand %prec BITWISE_AND
    | binaryOperationOrLower relationalEqualityOperator unaryOperationOrOperand %prec RELATIONAL_EQUALITY_OP
    | binaryOperationOrLower relationalSizeOperator unaryOperationOrOperand %prec RELATIONAL_SIZE_OP
    | binaryOperationOrLower addSub unaryOperationOrOperand %prec ADD_SUB
    | binaryOperationOrLower multDivRemainder unaryOperationOrOperand %prec MULT_DIV_REMAINDER
    | unaryOperationOrOperand
    ;

logicalOr:
    TK_OC_OR %prec LOGICAL_OR
    ;

logicalAnd:
    TK_OC_AND %prec LOGICAL_AND
    ;

bitwiseOr:
    '|' %prec BITWISE_OR
    ;

bitwiseXor:
    '^' %prec BITWISE_XOR
    ;

bitwiseAnd:
    '&' %prec BITWISE_AND
    ;

relationalEqualityOperator:
    TK_OC_EQ %prec RELATIONAL_EQUALITY_OP
    | TK_OC_NE %prec RELATIONAL_EQUALITY_OP
    ;

relationalSizeOperator:
    '<' %prec RELATIONAL_SIZE_OP
    | '>' %prec RELATIONAL_SIZE_OP
    | TK_OC_LE %prec RELATIONAL_SIZE_OP
    | TK_OC_GE %prec RELATIONAL_SIZE_OP
    ;

addSub:
    '+' %prec ADD_SUB
    | '-' %prec ADD_SUB
    ;

multDivRemainder:
    '*' %prec MULT_DIV_REMAINDER
    | '/' %prec MULT_DIV_REMAINDER
    | '%' %prec MULT_DIV_REMAINDER
    ;

unaryOperationOrOperand:
    expressionOperand
    | unaryOperatorList expressionOperand
    ;

unaryOperatorList:
    unaryOperatorList unaryOperator
    | unaryOperator
    ;

unaryOperator:
    '&' %prec UNARY_OPERATOR
    | '!' %prec UNARY_OPERATOR
    | '+' %prec UNARY_OPERATOR
    | '-' %prec UNARY_OPERATOR
    | '?' %prec UNARY_OPERATOR
    | '*' %prec UNARY_OPERATOR
    | '#' %prec UNARY_OPERATOR
    ;

expressionOperand: 
    TK_IDENTIFICADOR
    | TK_IDENTIFICADOR '[' expression ']'
    | literal
    | functionCall
    | grouping
    ;

grouping:
    openGrouping expression closeGrouping
    ;

openGrouping:
    '('
    ;

closeGrouping:
    ')' %prec GROUPING_CLOSE
    ;

optionalExpressionList:
    %empty
    | expressionList
    ;

expressionList:
    expression
    | expressionList ',' expression
    ;

%%

void exporta(void *arvore) {
    print_top_function((struct FunctionDef*) arvore);
}

void libera(void *arvore) {
    ;
}

void yyerror(char const *s) {
    printf("%s na linha %d\n", s, yylineno);
}
