%{
    #include<stdio.h>
    #include<stdlib.h>
%}

%start decl
%token CHAR COMMA FLOAT ID INT SEMI

%%

decl: type ID list;
list: COMMA ID list | SEMI;
type: INT | CHAR | FLOAT;

%%

int yyerror(char const *s) {
    printf("%s\n", s);
    return 1;
}

int main (int argc, char **argv)
{
    int ret = yyparse();
    if (ret != 0) {
        fprintf(stderr, "%d error found.\n", ret);
    }
    return ret;
}

