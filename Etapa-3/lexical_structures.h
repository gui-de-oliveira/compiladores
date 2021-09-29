// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include <stdbool.h> 

typedef struct ValorLexico ValorLexico;
typedef union TokenValue TokenValue;

enum TokenType {
	SPECIAL_CHAR, 		
	COMPOSITE_OPERATOR, 
	IDENTIFIER, 		
	LITERAL_INT, 		
	LITERAL_FLOAT, 		
	LITERAL_CHAR, 		
	LITERAL_BOOL, 		
	LITERAL_STRING,
	NODE
};

union TokenValue {
	int integer;		
	char character;		
	char* string;		
	float floating_point;
	bool boolean;		
	ValorLexico* children;		
};

struct ValorLexico {
	int line_number;
	enum TokenType token_type;
	union TokenValue token_value;
};
