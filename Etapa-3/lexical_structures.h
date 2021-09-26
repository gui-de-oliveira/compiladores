// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include <stdbool.h> 

// 2.1: tipo do token
enum TokenType {
	SPECIAL_CHAR, 		// caracteres especiais,
	COMPOSITE_OPERATOR, // operadores compostos,
	IDENTIFIER, 		// identificadores e

	// literais...
	// 2.1: Uma forma de implementar o valor do token para literais é utilizar dois campos:
	// um tipo de literal...
	LITERAL_INT, 		
	LITERAL_FLOAT, 		
	LITERAL_CHAR, 		
	LITERAL_BOOL, 		
	LITERAL_STRING, 	
};

// e o valor associado a ele através de uma construção union da linguagem C.

// 2.1: Os tokens de valores literais devem ter um tratamento especial, pois o valor do token
// deve ser convertido para o tipo apropriado (...
union TokenValue {
	int integer;			// inteiro int
	char character;			// caractere char	
	float floating_point;	// ponto-flutuante float
	bool boolean;			// booleano bool
	char* string;			// ou cadeia de caracteres char*)
};

// 2.1: O tipo do valor_lexico (e por consequência o valor que será retido) deve ser uma estrutura de dados que contém os seguintes campos:
struct ValorLexico {
	int line_number;			// 1. número da linha onde apareceu o lexema;
	enum TokenType token_type;	// 2. tipo do token (caracteres especiais, operadores compostos, identificadores e literais);
	union TokenValue token_value;	// 3. valor do token.
};
