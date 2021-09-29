// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include <stdbool.h> 
#include <stdlib.h> 
#include <stdio.h> 

typedef struct ValorLexico ValorLexico;
typedef struct ListElement ListElement;
typedef struct Node Node;
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
};

struct ValorLexico {
	int line_number;
	enum TokenType token_type;
	union TokenValue token_value;
	ListElement* children;
	ValorLexico* next;
};

struct ListElement {
	ValorLexico* value;
	ListElement* next;
};

void printLabels(ValorLexico* valorLexico);
void printDependencies(ValorLexico* valorLexico);
ValorLexico* createFunction(char* identifier);
ValorLexico* addAsNext(ValorLexico* parent, ValorLexico* child);