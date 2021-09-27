%{
    // Grupo L
    // Guilherme de Oliveira (00278301)
    // Jean Pierre Comerlatto Darricarrere (00182408)

    #include "abstract_syntax_tree.h"

    extern void *arvore;
    extern int yylineno;

    void exporta(void*);
    int yylex(void);
    void yyerror (char const *s);
%}

%union {
    struct ValorLexico valor_lexico;
    struct CommandList* optional_command_list;
    struct FunctionDef* optional_function_def;
    struct InitVar* init_var;
    struct Expression* expression;
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

%type<valor_lexico> literal
%type<valor_lexico> literal_int
%type<valor_lexico> shiftOperator

%type<expression> expression

%type<optional_command_list> varShift
%type<optional_command_list> localNameDefAssign
%type<optional_command_list> localDef
%type<optional_command_list> localNameDefList
%type<optional_command_list> simpleCommand
%type<optional_command_list> simpleCommandList
%type<optional_command_list> optionalSimpleCommandList
%type<optional_command_list> commandBlock
%type<optional_function_def> program
%type<optional_function_def> topLevelDefList
%type<optional_function_def> functionDef

%type<optional_command_list> varSet

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
        $$ = $2;
    }
    ;

optionalSimpleCommandList:
    %empty {
        $$ = NULL;
    }
    | simpleCommandList {
        $$ = $1;
    }
    ;

simpleCommandList:
    simpleCommand {
        $$ = $1;
    }
    | simpleCommandList simpleCommand {
        if($1 == NULL) {
            $$ = $2;
        } else {
            append_command($1, $2);
            $$ = $1;
        }
    }
    ;

simpleCommand:
    commandBlock ';' {
        $$ = $1;
    }
    | localDef ';' {
        $$ = $1;
    }
    | varSet ';' {
        $$ = $1;
    }
    | varShift ';' {
        $$ = $1;
    }
    | conditional ';' {
        $$ = NULL; // TO DO
    }
    | IO ';' {
        $$ = NULL; // TO DO
    }
    | functionCall ';' {
        $$ = NULL; // TO DO
    }
    | TK_PR_RETURN expression ';' {
        $$ = NULL; // TO DO
    }
    | TK_PR_CONTINUE ';' {
        $$ = NULL; // TO DO
    }
    | TK_PR_BREAK ';' {
        $$ = NULL; // TO DO
    }
    ;


literal:
    literal_int
    | TK_LIT_FLOAT
    | TK_LIT_FALSE
    | TK_LIT_TRUE
    | TK_LIT_CHAR
    | TK_LIT_STRING
    ;

literal_int:
    TK_LIT_INT
    ;


localDef:
    optionalStatic optionalConst type localNameDefList {
        $$ = $4;
    }
    ;

localNameDefList:
    TK_IDENTIFICADOR {
        $$ = NULL;
    }
    | localNameDefAssign {
        $$ = $1;
    }
    | localNameDefList ',' TK_IDENTIFICADOR {
        $$ = $1;
    }
    | localNameDefList ',' localNameDefAssign {
        if($1 == NULL) {
            $$ = $3;
        } else {
            append_command($1, $3);
            $$ = $1;
        }
    }
    ;

localNameDefAssign:
    TK_IDENTIFICADOR TK_OC_LE TK_IDENTIFICADOR {
        struct Identifier* left_identifier = new_identifier($1);
        struct Identifier* right_identifier = new_identifier($3);
        union InitVarData init_data;
        init_data.identifier = right_identifier;
        struct InitVar init_var = {.identifier = left_identifier, .init_type = IDENTIFIER_INIT, .init_data = init_data };
        union CommandData command = {.init_var = init_var};
        $$ = new_command(INIT_VAR, command);
    }
    | TK_IDENTIFICADOR TK_OC_LE literal {
        struct Identifier* left_identifier = new_identifier($1);
        struct Literal* literal = new_literal($3);
        union InitVarData init_data;
        init_data.literal = literal;
        struct InitVar init_var = {.identifier = left_identifier, .init_type = LITERAL_INIT, .init_data = init_data};
        union CommandData command = {.init_var = init_var};
        $$ = new_command(INIT_VAR, command);
    }
    ;

varShift:
    TK_IDENTIFICADOR shiftOperator literal_int {
        struct Identifier* identifier = new_identifier($1);
        union StorageAcessData storage_data = {.identifier = identifier};
        struct StorageAccess left_side = {.storage_type = IDENTIFIER_STORAGE, .storage_data = storage_data};
        struct Literal* right_side = new_literal($3);
        struct ShiftCommand shift = {.valor_lexico = $2, .left_side = left_side, .right_side = right_side};
        union CommandData command_data = {.shift_command = shift};
        $$ = new_command(SHIFT_COMMAND, command_data);
    }
    | TK_IDENTIFICADOR '[' expression ']' shiftOperator literal_int {
        struct Identifier* identifier = new_identifier($1);
        struct Expression* expression = $3;
        struct ArrayIndex* array_index = new_array_index(identifier, expression);
        union StorageAcessData storage_data = {.array_index = array_index};
        struct StorageAccess left_side = {.storage_type = ARRAY_INDEX_STORAGE, .storage_data = storage_data};
        struct Literal* right_side = new_literal($6);
        struct ShiftCommand shift = {.valor_lexico = $5, .left_side = left_side, .right_side = right_side};
        union CommandData command_data = {.shift_command = shift};
        $$ = new_command(SHIFT_COMMAND, command_data);
    }
    ;

varSet:
    TK_IDENTIFICADOR '=' expression {
        $$ = createSetVar($1);
    } 
    | TK_IDENTIFICADOR '[' expression ']' '=' expression {
        $$ = createSetVar($1);
    }
    ;

shiftOperator:
    TK_OC_SR %prec SHIFT_OPERATOR {
        $$ = $1;
    }
    | TK_OC_SL %prec SHIFT_OPERATOR {
        $$ = $1;
    }
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
    ternaryOperationOrLower {
        $$ = NULL; //TODO
    }
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
