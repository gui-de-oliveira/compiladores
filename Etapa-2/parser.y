%{
    #include <stdio.h>

	int get_line_number();

    int yylex(void);
    void yyerror (char const *s);
%}

%start initialSymbol

%token TK_PR_INT
%token TK_PR_FLOAT
%token TK_PR_BOOL
%token TK_PR_CHAR
%token TK_PR_STRING
%token TK_PR_IF
%token TK_PR_THEN
%token TK_PR_ELSE
%token TK_PR_WHILE
%token TK_PR_DO
%token TK_PR_INPUT
%token TK_PR_OUTPUT
%token TK_PR_RETURN
%token TK_PR_CONST
%token TK_PR_STATIC
%token TK_PR_FOREACH
%token TK_PR_FOR
%token TK_PR_SWITCH
%token TK_PR_CASE
%token TK_PR_BREAK
%token TK_PR_CONTINUE
%token TK_PR_CLASS
%token TK_PR_PRIVATE
%token TK_PR_PUBLIC
%token TK_PR_PROTECTED
%token TK_PR_END
%token TK_PR_DEFAULT
%token TK_OC_LE
%token TK_OC_GE
%token TK_OC_EQ
%token TK_OC_NE
%token TK_OC_AND
%token TK_OC_OR
%token TK_OC_SL
%token TK_OC_SR
%token TK_LIT_INT
%token TK_LIT_FLOAT
%token TK_LIT_FALSE
%token TK_LIT_TRUE
%token TK_LIT_CHAR
%token TK_LIT_STRING
%token TK_IDENTIFICADOR
%token TOKEN_ERRO

%nonassoc '(' ')' ';' 

%%


initialSymbol: ';' | declaration initialSymbol;

declaration: globalVariableDeclarationOrFunctionDeclaration | expression ';';

globalVariableDeclarationOrFunctionDeclaration: static type TK_IDENTIFICADOR solve;
type: TK_PR_INT | TK_PR_FLOAT | TK_PR_BOOL | TK_PR_CHAR | TK_PR_STRING ;
static: %empty | TK_PR_STATIC;

solve:
    globalVariableDeclaration |
    functionDeclaration ;

globalVariableDeclaration: headerArray ';' | headerArray ',' outroIdentificador ';';
outroIdentificador: TK_IDENTIFICADOR headerArray | TK_IDENTIFICADOR headerArray ',' outroIdentificador;
headerArray: %empty | '[' TK_LIT_INT ']'; 

functionDeclaration: '(' listOfParameters ')' '{' listOfCommandLines '}' ;
listOfParameters: %empty | parameter ;
parameter: const type TK_IDENTIFICADOR | const type TK_IDENTIFICADOR ',' parameter ;
const: %empty | TK_PR_CONST;

listOfCommandLines:  %empty | commandLine listOfCommandLines;

commandLine: 
      localDeclaration 
    | variableAssignOrExpression ';'
    | TK_PR_INPUT TK_IDENTIFICADOR ';';
    | TK_PR_OUTPUT anyValue ';'
    | TK_PR_BREAK ';'
    | TK_PR_CONTINUE ';'
    | TK_PR_RETURN expression ';'
    | TK_PR_IF '(' expression ')' '{' listOfCommandLines '}'
    | TK_PR_IF '(' expression ')' '{' listOfCommandLines '}' TK_PR_ELSE '{' listOfCommandLines '}'
    | TK_PR_WHILE '(' expression ')' TK_PR_DO '{' listOfCommandLines '}'
    ;

localDeclaration: static const type localIdentifiers ';';
localIdentifiers: TK_IDENTIFICADOR valueInitialization | TK_IDENTIFICADOR valueInitialization ',' localIdentifiers;
valueInitialization: %empty | TK_OC_LE initialValue;
initialValue: TK_IDENTIFICADOR | literalValue;
literalValue: TK_LIT_INT | TK_LIT_FLOAT | TK_LIT_FALSE | TK_LIT_TRUE | TK_LIT_CHAR | TK_LIT_STRING;

anyValue: TK_IDENTIFICADOR arraySelect | literalValue;
arraySelect: %empty | '[' expression ']'; 

variableAssignOrExpression: 
        TK_IDENTIFICADOR handle
    |   unaryOperator expression
    |   operadorLiteral tryOperator;

handle:
        '=' anyValue
    |   TK_OC_SL TK_LIT_INT
    |   TK_OC_SR TK_LIT_INT
    |   '(' listOfFnArgs ')' tryOperator
    |   '[' expression ']' arrayHandle
    |   tryOperator ;

arrayHandle:
    '=' anyValue
    |   TK_OC_SL TK_LIT_INT
    |   TK_OC_SR TK_LIT_INT
    |   tryOperator ;

expression: listOfUnaryOperators anyOperador tryOperator;

operadorFuncao: TK_IDENTIFICADOR '(' listOfFnArgs ')';
listOfFnArgs: %empty | fnArg;
fnArg: anyFnArg | anyFnArg ',' fnArg;

anyOperador: TK_LIT_INT | TK_LIT_FLOAT | TK_IDENTIFICADOR | TK_IDENTIFICADOR '[' expression ']' | operadorFuncao ;
operadorLiteral: TK_LIT_INT | TK_LIT_FLOAT;

anyFnArg: expression | TK_LIT_STRING | TK_LIT_CHAR | TK_LIT_TRUE | TK_LIT_FALSE ;

listOfUnaryOperators: %empty | unaryOperator listOfUnaryOperators;
unaryOperator: '+' | '-' | '!' | '?' | '&' | '*' | '#' ;

tryOperator: %empty | binaryOperator expression;
binaryOperator: '+' | '-' | '*' | '/' | '%' | '|' | '&' | '^' | TK_OC_NE | TK_OC_EQ | TK_OC_LE | TK_OC_GE | TK_OC_AND | TK_OC_OR ;

%%

void yyerror(char const *s) {
    printf("%s na linha %d\n", s, get_line_number());
}
