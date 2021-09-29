%{
    // Grupo L
    // Guilherme de Oliveira (00278301)
    // Jean Pierre Comerlatto Darricarrere (00182408)

	#include "lexical_structures.h"
	#include<stdio.h>

    extern void *arvore;
    extern int yylineno;

    void exporta(void*);
    int yylex(void);
    void yyerror (char const *s);
%}

%union {
    ValorLexico* valor_lexico;
    struct ListElement* list_element_ptr;
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

%type<valor_lexico> functionDef
%type<valor_lexico> topLevelDefList
%type<valor_lexico> program

%type<valor_lexico> commandBlock
%type<valor_lexico> simpleCommandList
%type<valor_lexico> optionalSimpleCommandList
%type<valor_lexico> simpleCommand
%type<valor_lexico> varSet

%type<valor_lexico> literal_int
%type<valor_lexico> literal
%type<valor_lexico> expressionOperand
%type<valor_lexico> unaryOperationOrOperand
%type<valor_lexico> binaryOperationOrLower
%type<valor_lexico> ternaryOperationOrLower
%type<valor_lexico> expression

%type<valor_lexico> functionCall

%type<list_element_ptr> optionalExpressionList
%type<list_element_ptr> expressionList

%%

program:
    %empty { 
        $$ = NULL; 
        arvore = NULL;
    }
    | topLevelDefList { 
        $$ = $1; 
        arvore = $1;
    }    
    ;

topLevelDefList:
    globalDef { $$ = NULL; }
    | functionDef { $$ =$1; }
    | topLevelDefList globalDef { $$ = $1; }
    | topLevelDefList functionDef { $$ = appendToValorLexico($1, $2); }
    ;

functionDef:
    optionalStatic type TK_IDENTIFICADOR '(' optionalParamList ')' commandBlock {
        ValorLexico* valorLexico = $3;
        ValorLexico* commandBlock = $7;
        valorLexico->children = appendToList(NULL, commandBlock);

        $$ = valorLexico;
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
    '{' optionalSimpleCommandList '}' { $$ = $2; }
    ;

optionalSimpleCommandList:
    %empty { $$ = NULL; }
    | simpleCommandList { $$ = $1; }
    ;

simpleCommandList:
    simpleCommand { $$ = $1; }
    | simpleCommandList simpleCommand { $$ = appendToValorLexico($1, $2);}
    ;

simpleCommand:
    commandBlock ';' { $$ = NULL; }
    | localDef ';' { $$ = NULL; }
    | varSet ';' { $$ = $1; }
    | varShift ';' { $$ = NULL; }
    | conditional ';' { $$ = NULL; }
    | IO ';' { $$ = NULL; }
    | functionCall ';' { $$ = $1; }
    | TK_PR_RETURN expression ';' { $$ = NULL; }
    | TK_PR_CONTINUE ';' { $$ = NULL; }
    | TK_PR_BREAK ';' { $$ = NULL; }
    ;


literal:
    literal_int { $$ = $1; }
    | TK_LIT_FLOAT { $$ = $1; }
    | TK_LIT_FALSE { $$ = $1; }
    | TK_LIT_TRUE { $$ = $1; }
    | TK_LIT_CHAR { $$ = $1; }
    | TK_LIT_STRING { $$ = $1; }
    ;

literal_int:
    TK_LIT_INT { $$ = $1; }
    ;


localDef:
    optionalStatic optionalConst type localNameDefList
    ;

localNameDefList:
    TK_IDENTIFICADOR
    | localNameDefAssign
    | localNameDefList ',' TK_IDENTIFICADOR
    | localNameDefList ',' localNameDefAssign
    ;

localNameDefAssign:
    TK_IDENTIFICADOR TK_OC_LE TK_IDENTIFICADOR
    | TK_IDENTIFICADOR TK_OC_LE literal
    ;

varShift:
    TK_IDENTIFICADOR shiftOperator literal_int
    | TK_IDENTIFICADOR '[' expression ']' shiftOperator literal_int
    ;

varSet:
    TK_IDENTIFICADOR '=' expression {
        ValorLexico* vlExpression = $3;

        ListElement* children = NULL;
        children = appendToList(children, $1);
        children = appendToList(children, vlExpression);

        ValorLexico* valorLexico = createSpecialCharValorLexico('=');
        valorLexico->children = children;

        $$ = valorLexico;
    }
    | TK_IDENTIFICADOR '[' expression ']' '=' expression { $$ = NULL; }
    ;

shiftOperator:
    TK_OC_SR %prec SHIFT_OPERATOR
    | TK_OC_SL %prec SHIFT_OPERATOR
    ;


functionCall:
    TK_IDENTIFICADOR '(' optionalExpressionList ')' {
        ValorLexico* identifier = $1;
        ValorLexico* functionCall = createStringValorLexico(LITERAL_STRING, "call g1");
        
        ValorLexico* children = NULL;
        children = appendToList(children, $3);
        functionCall->children = children;

        $$ = functionCall;
    }
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

expression: ternaryOperationOrLower { $$ = $1; };

ternaryOperationOrLower:
    binaryOperationOrLower ternaryOpen ternaryOperationOrLower ternaryClose ternaryOperationOrLower { $$ = NULL; }
    | binaryOperationOrLower { $$ = $1; }
    ;

ternaryOpen:
    '?' %prec TERNARY_OPEN
    ;

ternaryClose:
    ':' %prec TERNARY_CLOSE
    ;

binaryOperationOrLower:
    binaryOperationOrLower logicalOr unaryOperationOrOperand %prec LOGICAL_OR { $$ = NULL; }
    | binaryOperationOrLower logicalAnd unaryOperationOrOperand %prec LOGICAL_AND { $$ = NULL; }
    | binaryOperationOrLower bitwiseOr unaryOperationOrOperand %prec BITWISE_OR { $$ = NULL; }
    | binaryOperationOrLower bitwiseXor unaryOperationOrOperand %prec BITWISE_XOR { $$ = NULL; }
    | binaryOperationOrLower bitwiseAnd unaryOperationOrOperand %prec BITWISE_AND { $$ = NULL; }
    | binaryOperationOrLower relationalEqualityOperator unaryOperationOrOperand %prec RELATIONAL_EQUALITY_OP { $$ = NULL; }
    | binaryOperationOrLower relationalSizeOperator unaryOperationOrOperand %prec RELATIONAL_SIZE_OP { $$ = NULL; }
    | binaryOperationOrLower addSub unaryOperationOrOperand %prec ADD_SUB { $$ = NULL; }
    | binaryOperationOrLower multDivRemainder unaryOperationOrOperand %prec MULT_DIV_REMAINDER { $$ = NULL; }
    | unaryOperationOrOperand { $$ = $1; }
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
    expressionOperand { $$ = $1; }
    | unaryOperatorList expressionOperand { $$ = NULL; }
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
    TK_IDENTIFICADOR { $$ = $1; }
    | TK_IDENTIFICADOR '[' expression ']' {
        ValorLexico* identifier = $1;
        ValorLexico* expression = $3;

        ListElement* children = NULL;
        children = appendToList(children, identifier);
        children = appendToList(children, expression);

        ValorLexico* value = createStringValorLexico(LITERAL_STRING, "[]");
        value->children = children;

        $$ = value;
    }
    | literal  { $$ = $1; }
    | functionCall { $$ = $1; }
    | grouping { $$ = NULL; }
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
    %empty { $$ = NULL; }
    | expressionList { $$ = $1; } 
    ;

expressionList:
    expression { $$ = $1; }
    | expressionList ',' expression { $$ = appendToValorLexico($1, $3); }
    ;

%%

void exporta(void *arvore) {
    printDependencies((ValorLexico *) arvore);
    printLabels((ValorLexico *) arvore);
}

void libera(void *arvore) {
    ;
}

void yyerror(char const *s) {
    printf("%s na linha %d\n", s, yylineno);
}
