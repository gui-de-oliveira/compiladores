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

%right ')' ';'

%%

initialSymbol: 
    startCapturingGlobalVariableDeclaration |
    startFunctionDeclaration;

startCapturingGlobalVariableDeclaration: captureOptionalStaticGVB;
captureOptionalStaticGVB: captureTypeGVB | TK_PR_STATIC captureTypeGVB ;
captureTypeGVB: TK_PR_CHAR startCapturingIdGVB | TK_PR_INT startCapturingIdGVB | TK_PR_FLOAT startCapturingIdGVB | TK_PR_BOOL startCapturingIdGVB | TK_PR_STRING startCapturingIdGVB

startCapturingIdGVB: TK_IDENTIFICADOR captureArrayStartOrNextIdGVB ;
captureArrayStartOrNextIdGVB: '[' captureArrayGVB | captureNextIdOrEnd ;
captureArrayGVB: TK_LIT_INT ']' captureNextIdOrEnd;
captureNextIdOrEnd: ',' startCapturingIdGVB | endGVB;
endGVB: ';' ;

startFunctionDeclaration: captureOptionalStaticFD ;
captureOptionalStaticFD: captureReturnTypeFD | TK_PR_STATIC captureReturnTypeFD

captureReturnTypeFD:
    TK_PR_CHAR captureIdentifierFD |
    TK_PR_INT captureIdentifierFD |
    TK_PR_FLOAT captureIdentifierFD |
    TK_PR_BOOL captureIdentifierFD |
    TK_PR_STRING captureIdentifierFD ;

captureIdentifierFD: TK_IDENTIFICADOR '(' tryCaptureArgFD;

tryCaptureArgFD: ')' startCapturingCommandBlock | startCapturingArgFD;

startCapturingArgFD: tryCaptureArgStaticFD;
tryCaptureArgStaticFD: TK_PR_STATIC tryCaptureArgConstFD | tryCaptureArgConstFD;
tryCaptureArgConstFD: TK_PR_CONST captureArgTypeFD | captureArgTypeFD;
captureArgTypeFD: 
    TK_PR_CHAR captureArgIdFD |
     TK_PR_INT captureArgIdFD |
     TK_PR_FLOAT captureArgIdFD |
     TK_PR_BOOL captureArgIdFD |
     TK_PR_STRING captureArgIdFD ;

captureArgIdFD: TK_IDENTIFICADOR tryCaptureNextArgFD;
tryCaptureNextArgFD: ',' startCapturingArgFD | ')' startCapturingCommandBlock

startCapturingCommandBlock: '{' tryCaptureCommandLine;

tryCaptureCommandLine: '}' | startCapturingCommandLine;

startCapturingCommandLine: 
    startCapturingVariableDeclaration |
    startCapturingVariableAssignment |
    startCaptureOutput |
    captureInput |
    startCapturingFunctionCall |
    startCapturingShift |
    TK_PR_BREAK ';' tryCaptureCommandLine |
    TK_PR_CONTINUE ';' tryCaptureCommandLine |
    startCapturingExpression
    ;

startCapturingVariableDeclaration: tryCaptureArgStaticVD ;
tryCaptureArgStaticVD: tryCaptureArgConstVD | TK_PR_STATIC tryCaptureArgConstVD ;
tryCaptureArgConstVD: captureArgTypeVD | TK_PR_CONST captureArgTypeVD ;
captureArgTypeVD: 
    TK_PR_INT startCapturingIdentifierVD |
    TK_PR_FLOAT startCapturingIdentifierVD |
    TK_PR_BOOL startCapturingIdentifierVD |
    TK_PR_CHAR startCapturingIdentifierVD |
    TK_PR_STRING startCapturingIdentifierVD ;

startCapturingIdentifierVD: TK_IDENTIFICADOR tryCapturingInitializationVD ;
tryCapturingInitializationVD: TK_OC_LE startCaptureInitializationVD | tryCapturingNextIdentifierVD;
startCaptureInitializationVD: TK_IDENTIFICADOR tryCapturingNextIdentifierVD | captureLiteralInitializationValueVD;
captureLiteralInitializationValueVD: 
    TK_LIT_INT tryCapturingNextIdentifierVD |
    TK_LIT_FLOAT tryCapturingNextIdentifierVD |
    TK_LIT_FALSE tryCapturingNextIdentifierVD |
    TK_LIT_TRUE tryCapturingNextIdentifierVD |
    TK_LIT_CHAR tryCapturingNextIdentifierVD |
    TK_LIT_STRING tryCapturingNextIdentifierVD ;

tryCapturingNextIdentifierVD: ',' startCapturingIdentifierVD | genericEndCommandLine

startCapturingVariableAssignment: TK_IDENTIFICADOR tryCaptureArrayVA;
tryCaptureArrayVA: '[' TK_LIT_INT ']' '=' captureGenericValue | '=' captureGenericValue;

captureInput: TK_PR_INPUT TK_IDENTIFICADOR ';' tryCaptureCommandLine ;

startCaptureOutput: TK_PR_OUTPUT captureGenericValue ;

startCapturingFunctionCall: TK_IDENTIFICADOR '(' tryCapturingArgFC
tryCapturingArgFC: endFunctionCallCapture | captureArgIdentifier;
captureArgIdentifier: captureLiteralArgFC | TK_IDENTIFICADOR tryCapturingNextArgFC ;
captureLiteralArgFC:  
    TK_LIT_INT tryCapturingNextArgFC |
    TK_LIT_FLOAT tryCapturingNextArgFC |
    TK_LIT_FALSE tryCapturingNextArgFC |
    TK_LIT_TRUE tryCapturingNextArgFC |
    TK_LIT_CHAR tryCapturingNextArgFC |
    TK_LIT_STRING tryCapturingNextArgFC ;

tryCapturingNextArgFC: ',' captureArgIdentifier | endFunctionCallCapture ;
endFunctionCallCapture: ')' genericEndCommandLine ;

startCapturingShift: TK_IDENTIFICADOR tryCaptureArrayShift;
tryCaptureArrayShift: '[' TK_LIT_INT ']' captureShiftSymbol | captureShiftSymbol;
captureShiftSymbol: TK_OC_SR captureShiftValue | TK_OC_SL captureShiftValue;
captureShiftValue: TK_LIT_INT genericEndCommandLine;

startCapturingExpression: tryCaptureUnaryOperator;

tryCaptureUnaryOperator:
    '+' tryCaptureUnaryOperator |
    '-' tryCaptureUnaryOperator |
    '!' tryCaptureUnaryOperator |
    '?' tryCaptureUnaryOperator |
    '&' tryCaptureUnaryOperator |
    '*' tryCaptureUnaryOperator |
    '#' tryCaptureUnaryOperator |
    captureArgOperator;

captureArgOperator: captureLiteralArgOperator | TK_IDENTIFICADOR tryCaptureArray  ;

tryCaptureArray: 
    '[' TK_LIT_INT ']' tryCaptureOperatorEXP |
    '(' startCapturingArgsF  tryCaptureOperatorEXP |
    tryCaptureOperatorEXP ;

startCapturingArgsF: 
    TK_IDENTIFICADOR tryCatchingNextArgumentF |
    TK_LIT_FLOAT tryCatchingNextArgumentF |
    TK_LIT_INT tryCatchingNextArgumentF |
    stopCapturingArguments ;
    
tryCatchingNextArgumentF: ',' startCapturingArgsF | stopCapturingArguments;
stopCapturingArguments: ')'

captureLiteralArgOperator: 
    TK_LIT_INT tryCaptureOperatorEXP |
    TK_LIT_FLOAT tryCaptureOperatorEXP ;

tryCaptureOperatorEXP: 
    '+' tryCaptureUnaryOperator |
    '-' tryCaptureUnaryOperator |
    '*' tryCaptureUnaryOperator |
    '/' tryCaptureUnaryOperator |
    '%' tryCaptureUnaryOperator |
    '|' tryCaptureUnaryOperator |
    '&' tryCaptureUnaryOperator |
    '^' tryCaptureUnaryOperator |
    TK_OC_NE tryCaptureUnaryOperator |
    TK_OC_EQ tryCaptureUnaryOperator |
    TK_OC_LE tryCaptureUnaryOperator |
    TK_OC_GE tryCaptureUnaryOperator |
    TK_OC_AND tryCaptureUnaryOperator |
    TK_OC_OR tryCaptureUnaryOperator |
    genericEndCommandLine ;

captureGenericValue: TK_IDENTIFICADOR genericEndCommandLine | captureGenericLiteralValue ;
captureGenericLiteralValue: 
    TK_LIT_INT genericEndCommandLine |
    TK_LIT_FLOAT genericEndCommandLine |
    TK_LIT_FALSE genericEndCommandLine |
    TK_LIT_TRUE genericEndCommandLine |
    TK_LIT_CHAR genericEndCommandLine |
    TK_LIT_STRING genericEndCommandLine ;

genericEndCommandLine: ';' tryCaptureCommandLine ;

%%

void yyerror(char const *s) {
    printf("%s na linha %d\n", s, get_line_number());
}
