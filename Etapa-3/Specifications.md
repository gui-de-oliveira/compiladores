# Especificações Etapa 3

## 1. Introdução

A terceira etapa do projeto de compilador para a linguagem consiste na criaçäo da árvore sintática abstrata (Abstract Syntax Tree — AST) baseada no programa de entrada.

A árvore deve ser obrigatoriamente criada na medida que as regras semånticas säo executadas...

- [ ] [Criar teste para checar se árvore está sendo alocada dinamicamente](https://github.com/GuiOliveira98/compiladores/issues/7)

e deve ser mantida em memória mesmo após o fim da análise sintática, ou seja, quando yyparse retornar.

- [ ] [Criar teste para checar se está sendo mantida em memória](https://github.com/GuiOliveira98/compiladores/issues/8)

## 2. Funcionalidades Necessárias

### 2.1 Associação de valor ao token (yylval)

Nesta etapa, deve-se associar um valor para alguns tokens.

Esta associação deve ser feita no analisador léxico, ou seja, no arquivo scanner.l.

Ela é realizada através do uso da variável global yylval que é usada pelo flex para dar um “valor” ao token...

```c
// Exemplo de uso de yylval no scanner.l
{pattern_integer} {
    yylval = 1;
    return TK_LIT_INT;
}
```

em complemento ao uso das constantes de identificação (comando %token).

```c
// Exemplo do comando %token no parser.y
%token TK_LIT_INT
```

Como esta variável global pode ser configurada com a diretiva %union, sugere-se o uso do campo com o nome valor_lexico para a associação.

```c
// Exemplo de uso da diretiva %union no arquivo parser.y
%union {
    valor_lexico_t* valor_lexico;
}
```

Portanto, a associação deverá ser feita através de uma atribuição para a variável yylval.valor_lexico.

```c
{pattern_integer} {
    yylval.valor_lexico = 1;
    return TK_LIT_INT;
}
```

O tipo do valor_lexico (e por consequência o valor que será retido) deve ser uma estrutura de dados que contém os seguintes campos:

1. número da linha onde apareceu o lexema;
1. tipo do token (caracteres especiais, operadores compostos, identificadores e literais);
1. valor do token.

Não há necessidade de lidar com palavras-reservadas.

- [x] [Implementação do tipo valor_lexico_t](https://github.com/GuiOliveira98/compiladores/issues/9)

O valor do token deve ser uma cadeia de caracteres (duplicada com strdup a partir de yytext) para os tokens de caracteres especiais, operadores compostos, identificadores.

```c
// Exemplo da setagem do valor de um caractere especial
{pattern_special_char} {
    ... //inicialização do valor lexico aqui

    yylval.valor_lexico.token_value = strdup(yytext);

    return get_ascii_value();
}
```

Os tokens de valores literais devem ter um tratamento especial, pois o valor do token deve ser convertido para o tipo apropriado (inteiro int, ponto-flutuante float, caractere char, booleano bool (ou int) ou cadeia de caracteres char\*).

A conversão é feita com funções tais como atoi e atof.

```c
// Exemplo da setagem de um valor literal
{pattern_integer} {
    ... //inicialização do valor lexico aqui

    yylval.valor_lexico.token_value = atoi(yytext);

    return TK_LIT_INT;
}
```

Os tipos caractere e cadeia de caracteres não devem conter aspas (simples ou duplas) no campo valor (e devem ser duplicados com strdup).

Uma forma de implementar o valor do token para literais é utilizar dois campos: um tipo de literal e o valor associado a ele através de uma construção union da linguagem C.

```c
enum LiteralType {
    INT,
    CHAR
} literalType;

union LiteralValue {
    int vi;
    char vc;
} literalValue;

struct LiteralValue {
  literalType type;
  literalValue value;
} literalValue;

```

[Qual a diferença de union e struct?](https://stackoverflow.com/questions/346536/difference-between-a-structure-and-a-union)

- [x] [Captura de todos os valores de token](https://github.com/GuiOliveira98/compiladores/issues/10)
