%{
	// Grupo L
	// Guilherme de Oliveira (00278301)
	// Jean Pierre Comerlatto Darricarrere (00182408)

	#include "lexical_structures.h"
	#include "parser.tab.h"

	// Variable definition
	int line_number = 1;
	int state_before_comment;
%}

%option yylineno

%x MULTILINE_COMMENT

pattern_line_comment "//".*

pattern_aux_char [,;:()\[\]{}.$]
pattern_special_char [+\-*/<>=!&|%#^\?]

pattern_alphabetic [a-zA-Z_]
pattern_alphanumeric [a-zA-Z_0-9]
pattern_identifier {pattern_alphabetic}{pattern_alphanumeric}*

pattern_digit [0-9]
pattern_integer {pattern_digit}+
pattern_signed_int [+-]?{pattern_integer}
pattern_exponent [eE]{pattern_signed_int}
pattern_float {pattern_integer}\.{pattern_integer}{pattern_exponent}?

pattern_char '.'
pattern_string \"(\\.|[^"\r\n])*\"
/* " <-- This messes up the editor color coding */

%%

{pattern_line_comment} {
	/* Discards anything between // and new line */
};

"/*" {
	state_before_comment = YY_START;
	BEGIN(MULTILINE_COMMENT);
}

"int" return TK_PR_INT;
"float" return TK_PR_FLOAT;
"bool" return TK_PR_BOOL;
"char" return TK_PR_CHAR;
"string" return TK_PR_STRING;
"then" return TK_PR_THEN;
"else" return TK_PR_ELSE;
"do" return TK_PR_DO;
"if" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_IF); return TK_PR_IF; }
"for" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_FOR); return TK_PR_FOR; }
"while" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_WHILE); return TK_PR_WHILE; }
"input" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_INPUT); return TK_PR_INPUT; }
"output" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_OUTPUT); return TK_PR_OUTPUT; }
"return" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_RETURN); return TK_PR_RETURN; }
"break" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_BREAK); return TK_PR_BREAK; }
"continue" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_CONTINUE); return TK_PR_CONTINUE; }
"const" return TK_PR_CONST;
"static" return TK_PR_STATIC;
"foreach" return TK_PR_FOREACH;
"switch" return TK_PR_SWITCH;
"case" return TK_PR_CASE;
"class" return TK_PR_CLASS;
"private" return TK_PR_PRIVATE;
"public" return TK_PR_PUBLIC;
"protected" return TK_PR_PROTECTED;

"false" { 
	yylval.valor_lexico = createBoolValorLexico(false);
	return TK_LIT_FALSE;
}

"true" { 
	yylval.valor_lexico = createBoolValorLexico(true);
	return TK_LIT_TRUE;
}

"end" return TK_PR_END;
"default" return TK_PR_DEFAULT;

{pattern_aux_char} {
	return yytext[0];
}

{pattern_special_char} {
	yylval.valor_lexico = createSpecialCharValorLexico(yytext[0]);
	return yytext[0];
}

"<=" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_LESS_EQUAL); return TK_OC_LE; }
">=" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_MORE_EQUAL); return TK_OC_GE; }
"==" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_EQUAL); return TK_OC_EQ; }
"!=" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_UNEQUAL); return TK_OC_NE; }
"&&" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_BOOL_AND); return TK_OC_AND; }
"||" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_BOOL_OR); return TK_OC_OR; }
">>" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_RIGHT_SHIFT); return TK_OC_SR; }
"<<" { yylval.valor_lexico = createStringValorLexico(SPECIAL_KEYWORD, SK_LEFT_SHIFT); return TK_OC_SL; }

{pattern_identifier} {
	char* string = strdup(yytext);
	yylval.valor_lexico = createStringValorLexico(IDENTIFIER, string);
	return TK_IDENTIFICADOR;
}
{pattern_integer} {
	yylval.valor_lexico = createIntegerValorLexico(atoi(yytext));
	return TK_LIT_INT;
}
{pattern_float} {
	yylval.valor_lexico = createFloatValorLexico(atof(yytext));
	return TK_LIT_FLOAT;
}

{pattern_char} {
	yylval.valor_lexico = createCharValorLexico(yytext[1]);
	return TK_LIT_CHAR;
}

{pattern_string} {
	char* string = strdup(yytext + 1);
	string[ strlen(string) - 1 ] = '\0';
	yylval.valor_lexico = createStringValorLexico(LITERAL_STRING, string);
	return TK_LIT_STRING;
}

<INITIAL,MULTILINE_COMMENT>\n|(\r\n) {
	++line_number;
	/* Increment line counter regardless of state, takes precedence over next rule due to order. */
}
<MULTILINE_COMMENT>. {
	/* Eat everything preceding the closing multi-line comment (one char at a time) */
}

<MULTILINE_COMMENT>"*/" {
	/* End the multi-line comment. Takes precedence to former two due to match length (two chars vs one). */
	BEGIN(state_before_comment);
}

[[:blank:]] {
	/* Discard tabs and whitespaces. Equivalent with [ \t] */
}
. {
	/* Everything else should be an error. */
	return TOKEN_ERRO;
}

%%