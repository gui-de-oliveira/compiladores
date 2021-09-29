// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include <stdbool.h> 
#include <stdlib.h> 
#include <stdio.h> 
#include <string.h>

extern const char* SK_RETURN;
extern const char* SK_CONTINUE;
extern const char* SK_BREAK;
extern const char* SK_LEFT_SHIFT;
extern const char* SK_RIGHT_SHIFT;
extern const char* SK_INPUT;
extern const char* SK_OUTPUT;
extern const char* SK_IF;
extern const char* SK_FOR;
extern const char* SK_WHILE;
extern const char* SK_TERNARY;
extern const char* SK_BOOL_OR;
extern const char* SK_BOOL_AND;
extern const char* SK_EQUAL;
extern const char* SK_UNEQUAL;
extern const char* SK_LESS_EQUAL;
extern const char* SK_MORE_EQUAL;
extern const char* SK_ARRAY;

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
