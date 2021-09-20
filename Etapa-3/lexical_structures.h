// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include <stdbool.h> 

typedef enum TokenType {
	SPECIAL_CHAR, //string
	COMPOSITE_OPERATOR, //string
	IDENTIFIER, //string
	LITERAL_INT, //integer
	LITERAL_FLOAT, //floating_point
	LITERAL_CHAR, //character
	LITERAL_BOOL, //boolean
	LITERAL_STRING, //string
} token_type_t;

typedef union TokenValue {
	char character;
	int integer;
	float floating_point;
	bool boolean;
	char* string;
} token_value_t;

typedef struct ValorLexico {
	int line_number;
	token_type_t token_type;
	token_value_t token_value;
} valor_lexico_t;
