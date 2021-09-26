%{
    // Grupo L
    // Guilherme de Oliveira (00278301)
    // Jean Pierre Comerlatto Darricarrere (00182408)

    #include "abstract_syntax_tree.h"

    extern void *arvore;
    extern int yylineno;

    dummy_function_def_t DUMMY = { .label = "DUMMY"};

    int yylex(void);
    void yyerror (char const *s);
%}

%union {
    valor_lexico_t valor_lexico;
    dummy_function_def_t dummy_function_def;
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

%type<dummy_function_def> program
%type<dummy_function_def> topLevelDefList
%type<dummy_function_def> functionDef

%%

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

functionDef:
    optionalStatic type TK_IDENTIFICADOR '(' optionalParamList ')' commandBlock {
        valor_lexico_t x = $3;
        dummy_function_def_t function = { .label = x.token_value.string, .next_function = NULL};
        $$ = function;
    }
    ;


commandBlock:
    '{' optionalSimpleCommandList '}'
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
    TK_OC_SR
    | TK_OC_SL
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
    optionalOperator expressionOperand
    | optionalOperator '(' expression ')'
    ;

optionalOperator:
    %empty
    | optionalOperator unaryOperator
    | expression binaryOperator
    | expression '?' expression ':'
    ;

expressionOperand: 
    TK_IDENTIFICADOR
    | TK_IDENTIFICADOR '[' expression ']'
    | literal
    | functionCall
    ;

unaryOperator:
    '&'
    | '!'
    | '+'
    | '-'
    | '?'
    | '*'
    | '#'
    ;

binaryOperator:
    TK_OC_LE
    | TK_OC_GE
    | TK_OC_EQ
    | TK_OC_NE
    | TK_OC_AND
    | TK_OC_OR
    | '<'
    | '>'
    | '+'
    | '-'
    | '*'
    | '/'
    | '%'
    | '|'
    | '&'
    | '^'
    ;

optionalExpressionList:
    %empty
    | expressionList
    ;

expressionList:
    expression
    | expressionList ',' expression
    ;


topLevelDefList:
    globalDef {
        $$ = DUMMY;
    }
    | functionDef {
        $$ = $1;
    }
    | topLevelDefList globalDef {
        $$ = DUMMY;
    }
    | topLevelDefList functionDef {
        $$ = DUMMY;
    }
    ;

program:
    %empty {
        $$ = DUMMY;
    }
    | topLevelDefList {
        dummy_function_def_t function = $1;
        if(strcmp(function.label, "DUMMY") != 0){
            printf("%p [label=\"%s\"];", &function, function.label);
        }
        $$ = $1;
    }
    ;

%%

void exporta(void *arvore) {
    ;
}

void libera(void *arvore) {
    ;
}

void yyerror(char const *s) {
    printf("%s na linha %d\n", s, yylineno);
}
