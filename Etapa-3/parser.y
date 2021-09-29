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
%type<valor_lexico> arrayAccess

%type<valor_lexico> literal_int
%type<valor_lexico> literal
%type<valor_lexico> expressionOperand
%type<valor_lexico> unaryOperationOrOperand
%type<valor_lexico> binaryOperationOrLower
%type<valor_lexico> ternaryOperationOrLower
%type<valor_lexico> expression
%type<valor_lexico> grouping

%type<valor_lexico> logicalOr
%type<valor_lexico> logicalAnd
%type<valor_lexico> bitwiseOr
%type<valor_lexico> bitwiseXor
%type<valor_lexico> bitwiseAnd
%type<valor_lexico> relationalEqualityOperator
%type<valor_lexico> relationalSizeOperator
%type<valor_lexico> addSub
%type<valor_lexico> multDivRemainder
%type<valor_lexico> unaryOperatorList
%type<valor_lexico> unaryOperator

%type<valor_lexico> functionCall

%type<valor_lexico> optionalExpressionList
%type<valor_lexico> expressionList

%type<valor_lexico> IO

%type<valor_lexico> varShift
%type<valor_lexico> shiftOperator

%type<valor_lexico> localDef
%type<valor_lexico> localNameDefList
%type<valor_lexico> localNameDefAssign

%type<valor_lexico> conditional

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
    commandBlock ';' { $$ = $1; }
    | localDef ';' { $$ = $1; }
    | varSet ';' { $$ = $1; }
    | varShift ';' { $$ = $1; }
    | conditional ';' { $$ = $1; }
    | IO ';' { $$ = $1; }
    | functionCall ';' { $$ = $1; }
    | TK_PR_RETURN expression ';' { 
        ValorLexico* valorLexico = createStringValorLexico(SPECIAL_KEYWORD, SK_RETURN);
        valorLexico->children = appendToList(NULL, $2);
        $$ = valorLexico;
    }
    | TK_PR_CONTINUE ';' { $$ = createStringValorLexico(SPECIAL_KEYWORD, SK_CONTINUE); }
    | TK_PR_BREAK ';' { $$ = createStringValorLexico(SPECIAL_KEYWORD, SK_BREAK); }
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
    optionalStatic optionalConst type localNameDefList { $$ = $4; }
    ;

localNameDefList:
    TK_IDENTIFICADOR { $$ = NULL; }
    | localNameDefAssign { $$ = $1;}
    | localNameDefList ',' TK_IDENTIFICADOR { $$ = $1; }
    | localNameDefList ',' localNameDefAssign { $$ = appendToValorLexico($1, $3); }
    ;

localNameDefAssign:
    TK_IDENTIFICADOR TK_OC_LE TK_IDENTIFICADOR {
        ValorLexico* id = createStringValorLexico(SPECIAL_KEYWORD, SK_LESS_EQUAL);
        id->children = appendToList(NULL, $1);
        id->children = appendToList(id->children, $3);

        $$ = id;
    }
    | TK_IDENTIFICADOR TK_OC_LE literal{
        ValorLexico* id = createStringValorLexico(SPECIAL_KEYWORD, SK_LESS_EQUAL);
        id->children = appendToList(NULL, $1);
        id->children = appendToList(id->children, $3);

        $$ = id;
    }
    ;

varShift:
    TK_IDENTIFICADOR shiftOperator literal_int {
        ListElement* children = appendToList(NULL, $1);
        children = appendToList(children, $3);
        $2->children = children;
        $$ = $2;
    }
    | arrayAccess shiftOperator literal_int {
        ListElement* children = appendToList(NULL, $1);
        children = appendToList(children, $3);
        ValorLexico* shift_operator = $2;
        shift_operator->children = children;
        $$ = shift_operator;
    }
    ;

varSet:
    TK_IDENTIFICADOR '=' expression {
        ListElement* children = appendToList(NULL, $1);
        children = appendToList(children, $3);
        ValorLexico* set_operator = createSpecialCharValorLexico('=');
        set_operator->children = children;
        $$ = set_operator;
    }
    | arrayAccess '=' expression {
        ListElement* children = appendToList(NULL, $1);
        children = appendToList(children, $3);
        ValorLexico* set_operator = createSpecialCharValorLexico('=');
        set_operator->children = children;
        $$ = set_operator;
    }
    ;

shiftOperator:
    TK_OC_SR %prec SHIFT_OPERATOR { $$ = createStringValorLexico(SPECIAL_KEYWORD, SK_RIGHT_SHIFT); } ;
    | TK_OC_SL %prec SHIFT_OPERATOR { $$ = createStringValorLexico(SPECIAL_KEYWORD, SK_LEFT_SHIFT); } ;
    ;


functionCall:
    TK_IDENTIFICADOR '(' optionalExpressionList ')' {
        ValorLexico* identifier = $1;

        char* function_name = identifier->token_value.string;
        int length = strlen(function_name);
        char* function_call = (char*) malloc(length + 5);
        function_call[0] = '\0';
        strcat(function_call, "call ");
        strcat(function_call, function_name);

        freeValorLexico(identifier);

        ValorLexico* functionCall = createStringValorLexico(IDENTIFIER, function_call);
        functionCall->children = appendToList(functionCall->children, $3);

        $$ = functionCall;
    }
    ;


IO:
    TK_PR_INPUT TK_IDENTIFICADOR { 
        ValorLexico* valorLexico = createStringValorLexico(SPECIAL_KEYWORD, SK_INPUT);
        valorLexico->children = appendToList(NULL, $2);
        $$ = valorLexico;
    }
    | TK_PR_OUTPUT TK_IDENTIFICADOR {
        ValorLexico* valorLexico = createStringValorLexico(SPECIAL_KEYWORD, SK_OUTPUT);
        valorLexico->children = appendToList(NULL, $2);
        $$ = valorLexico;
    }
    | TK_PR_OUTPUT literal {
        ValorLexico* valorLexico = createStringValorLexico(SPECIAL_KEYWORD, SK_OUTPUT);
        valorLexico->children = appendToList(NULL, $2);
        $$ = valorLexico;
    }
    ;


conditional:
    TK_PR_IF '(' expression ')' commandBlock {
        ValorLexico* id = createStringValorLexico(SPECIAL_KEYWORD, SK_IF);
        id->children = appendToList(NULL, $3);
        id->children = appendToList(id->children, $5);

        $$ = id; 
    }
    | TK_PR_IF '(' expression ')' commandBlock TK_PR_ELSE commandBlock {
        ValorLexico* id = createStringValorLexico(SPECIAL_KEYWORD, SK_IF);
        id->children = appendToList(NULL, $3);
        id->children = appendToList(id->children, $5);
        id->children = appendToList(id->children, $7);

        $$ = id; 
    }
    | TK_PR_FOR '(' varSet ':' expression ':' varSet ')' commandBlock  {
        ValorLexico* id = createStringValorLexico(SPECIAL_KEYWORD, SK_FOR);
        id->children = appendToList(NULL, $3);
        id->children = appendToList(id->children, $5);
        id->children = appendToList(id->children, $7);
        id->children = appendToList(id->children, $9);

        $$ = id; 
    }
    | TK_PR_WHILE '(' expression ')' TK_PR_DO commandBlock {
        ValorLexico* id = createStringValorLexico(SPECIAL_KEYWORD, SK_WHILE);
        id->children = appendToList(NULL, $3);
        id->children = appendToList(id->children, $6);

        $$ = id; 
    }
    ;

expression: ternaryOperationOrLower { $$ = $1; };

ternaryOperationOrLower:
    binaryOperationOrLower ternaryOpen ternaryOperationOrLower ternaryClose ternaryOperationOrLower {
        ListElement* children = NULL;
        children = appendToList(children, $1);
        children = appendToList(children, $3);
        children = appendToList(children, $5);
        ValorLexico* value = createStringValorLexico(SPECIAL_KEYWORD, SK_TERNARY);
        value->children = children;
        $$ = value;
     }
    | binaryOperationOrLower { $$ = $1; }
    ;

ternaryOpen:
    '?' %prec TERNARY_OPEN
    ;

ternaryClose:
    ':' %prec TERNARY_CLOSE
    ;

binaryOperationOrLower:
    binaryOperationOrLower logicalOr unaryOperationOrOperand %prec LOGICAL_OR {
        ListElement* children = NULL;
        children = appendToList(children, $1);
        children = appendToList(children, $3);
        ValorLexico* value = $2;
        value->children = children;
        $$ = value;
     }
    | binaryOperationOrLower logicalAnd unaryOperationOrOperand %prec LOGICAL_AND {
        ListElement* children = NULL;
        children = appendToList(children, $1);
        children = appendToList(children, $3);
        ValorLexico* value = $2;
        value->children = children;
        $$ = value;
     }
    | binaryOperationOrLower bitwiseOr unaryOperationOrOperand %prec BITWISE_OR {
        ListElement* children = NULL;
        children = appendToList(children, $1);
        children = appendToList(children, $3);
        ValorLexico* value = $2;
        value->children = children;
        $$ = value;
     }
    | binaryOperationOrLower bitwiseXor unaryOperationOrOperand %prec BITWISE_XOR {
        ListElement* children = NULL;
        children = appendToList(children, $1);
        children = appendToList(children, $3);
        ValorLexico* value = $2;
        value->children = children;
        $$ = value;
     }
    | binaryOperationOrLower bitwiseAnd unaryOperationOrOperand %prec BITWISE_AND {
        ListElement* children = NULL;
        children = appendToList(children, $1);
        children = appendToList(children, $3);
        ValorLexico* value = $2;
        value->children = children;
        $$ = value;
     }
    | binaryOperationOrLower relationalEqualityOperator unaryOperationOrOperand %prec RELATIONAL_EQUALITY_OP {
        ListElement* children = NULL;
        children = appendToList(children, $1);
        children = appendToList(children, $3);
        ValorLexico* value = $2;
        value->children = children;
        $$ = value;
     }
    | binaryOperationOrLower relationalSizeOperator unaryOperationOrOperand %prec RELATIONAL_SIZE_OP {
        ListElement* children = NULL;
        children = appendToList(children, $1);
        children = appendToList(children, $3);
        ValorLexico* value = $2;
        value->children = children;
        $$ = value;
     }
    | binaryOperationOrLower addSub unaryOperationOrOperand %prec ADD_SUB {
        ListElement* children = NULL;
        children = appendToList(children, $1);
        children = appendToList(children, $3);
        ValorLexico* value = $2;
        value->children = children;
        $$ = value;
     }
    | binaryOperationOrLower multDivRemainder unaryOperationOrOperand %prec MULT_DIV_REMAINDER {
        ListElement* children = NULL;
        children = appendToList(children, $1);
        children = appendToList(children, $3);
        ValorLexico* value = $2;
        value->children = children;
        $$ = value;
     }
    | unaryOperationOrOperand { $$ = $1; }
    ;

logicalOr:
    TK_OC_OR %prec LOGICAL_OR { $$ = createStringValorLexico(SPECIAL_KEYWORD, SK_BOOL_OR); }
    ;

logicalAnd:
    TK_OC_AND %prec LOGICAL_AND { $$ = createStringValorLexico(SPECIAL_KEYWORD, SK_BOOL_AND); }
    ;

bitwiseOr:
    '|' %prec BITWISE_OR { $$ = createSpecialCharValorLexico('|'); }
    ;

bitwiseXor:
    '^' %prec BITWISE_XOR { $$ = createSpecialCharValorLexico('^'); }
    ;

bitwiseAnd:
    '&' %prec BITWISE_AND { $$ = createSpecialCharValorLexico('&'); }
    ;

relationalEqualityOperator:
    TK_OC_EQ %prec RELATIONAL_EQUALITY_OP { $$ = createStringValorLexico(SPECIAL_KEYWORD, SK_EQUAL); }
    | TK_OC_NE %prec RELATIONAL_EQUALITY_OP { $$ = createStringValorLexico(SPECIAL_KEYWORD, SK_UNEQUAL); }
    ;

relationalSizeOperator:
    '<' %prec RELATIONAL_SIZE_OP { $$ = createSpecialCharValorLexico('<'); }
    | '>' %prec RELATIONAL_SIZE_OP { $$ = createSpecialCharValorLexico('>'); }
    | TK_OC_LE %prec RELATIONAL_SIZE_OP { $$ = createStringValorLexico(SPECIAL_KEYWORD, SK_LESS_EQUAL); }
    | TK_OC_GE %prec RELATIONAL_SIZE_OP { $$ = createStringValorLexico(SPECIAL_KEYWORD, SK_MORE_EQUAL); }
    ;

addSub:
    '+' %prec ADD_SUB { $$ = createSpecialCharValorLexico('+'); }
    | '-' %prec ADD_SUB { $$ = createSpecialCharValorLexico('-'); }
    ;

multDivRemainder:
    '*' %prec MULT_DIV_REMAINDER { $$ = createSpecialCharValorLexico('*'); }
    | '/' %prec MULT_DIV_REMAINDER { $$ = createSpecialCharValorLexico('/'); }
    | '%' %prec MULT_DIV_REMAINDER { $$ = createSpecialCharValorLexico('%'); }
    ;

unaryOperationOrOperand:
    expressionOperand { $$ = $1; }
    | unaryOperatorList expressionOperand {
        ValorLexico* top_value = $1;
        ValorLexico* bottom_value = top_value;
        while(bottom_value->children != NULL) {
            bottom_value = bottom_value->children->value;
        }
        bottom_value->children = appendToList(bottom_value->children, $2);

        $$ = top_value;
        }
    ;

unaryOperatorList:
    unaryOperatorList unaryOperator {
        ListElement* children = NULL;
        children = appendToList(children, $2);

        ValorLexico* value = $1;
        value->children = children;

        $$ = value;
    }
    | unaryOperator { $$ = $1; }
    ;

unaryOperator:
    '&' %prec UNARY_OPERATOR { $$ = createSpecialCharValorLexico('&'); }
    | '!' %prec UNARY_OPERATOR { $$ = createSpecialCharValorLexico('!'); }
    | '+' %prec UNARY_OPERATOR { $$ = createSpecialCharValorLexico('+'); }
    | '-' %prec UNARY_OPERATOR { $$ = createSpecialCharValorLexico('-'); }
    | '?' %prec UNARY_OPERATOR { $$ = createSpecialCharValorLexico('?'); }
    | '*' %prec UNARY_OPERATOR { $$ = createSpecialCharValorLexico('*'); }
    | '#' %prec UNARY_OPERATOR { $$ = createSpecialCharValorLexico('#'); }
    ;

expressionOperand: 
    TK_IDENTIFICADOR { $$ = $1; }
    | arrayAccess { $$ = $1; }
    | literal { $$ = $1; }
    | functionCall { $$ = $1; }
    | grouping { $$ = $1; }
    ;

arrayAccess:
    TK_IDENTIFICADOR '[' expression ']' {
        ValorLexico* identifier = $1;
        ListElement* children = appendToList(NULL, identifier);
        ValorLexico* expression = $3;
        children = appendToList(children, expression);
        ValorLexico* array_access = createStringValorLexico(SPECIAL_KEYWORD, SK_ARRAY);
        array_access->children = children;
        $$ = array_access;
    }
    ;

grouping:
    openGrouping expression closeGrouping { $$ = $2; }
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
