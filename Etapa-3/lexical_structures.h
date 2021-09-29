// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include <stdbool.h> 
#include <stdlib.h> 
#include <stdio.h> 
#include <string.h>

extern char* SK_RETURN;
extern char* SK_CONTINUE;
extern char* SK_BREAK;
extern char* SK_LEFT_SHIFT;
extern char* SK_RIGHT_SHIFT;
extern char* SK_INPUT;
extern char* SK_OUTPUT;
extern char* SK_IF;
extern char* SK_FOR;
extern char* SK_WHILE;
extern char* SK_TERNARY;
extern char* SK_BOOL_OR;
extern char* SK_BOOL_AND;
extern char* SK_EQUAL;
extern char* SK_UNEQUAL;
extern char* SK_LESS_EQUAL;
extern char* SK_MORE_EQUAL;
extern char* SK_ARRAY;

typedef struct ValorLexico ValorLexico;
typedef struct ListElement ListElement;
typedef struct Node Node;
typedef union TokenValue TokenValue;

enum TokenType {
	SPECIAL_CHAR,
	SPECIAL_KEYWORD,
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

ValorLexico* createIntegerValorLexico(int integer);
ValorLexico* createStringValorLexico(enum TokenType type, char* string);
ValorLexico* createSpecialCharValorLexico(char character);
ValorLexico* createFloatValorLexico(float floating_point);
ValorLexico* createCharValorLexico(char character);
ValorLexico* createBoolValorLexico(bool boolean);

ValorLexico* appendToValorLexico(ValorLexico* parent, ValorLexico* child);
ListElement* appendToList(ListElement* parent, ValorLexico* item);

void freeValorLexico(ValorLexico* lexical_value);
void freeListElement(ListElement* list_element);
