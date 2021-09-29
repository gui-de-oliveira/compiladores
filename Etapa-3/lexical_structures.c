// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include "lexical_structures.h"

const char* SK_RETURN = "return";
const char* SK_CONTINUE = "continue";
const char* SK_BREAK = "break";
const char* SK_LEFT_SHIFT = "<<";
const char* SK_RIGHT_SHIFT = ">>";
const char* SK_INPUT = "input";
const char* SK_OUTPUT = "output";
const char* SK_IF = "if";
const char* SK_FOR = "for";
const char* SK_WHILE = "while";
const char* SK_TERNARY = "?:";
const char* SK_BOOL_OR = "||";
const char* SK_BOOL_AND = "&&";
const char* SK_EQUAL = "==";
const char* SK_UNEQUAL = "!=";
const char* SK_LESS_EQUAL = "<=";
const char* SK_MORE_EQUAL = ">=";
const char* SK_ARRAY = "[]";

void printDependencies(ValorLexico* valorLexico){
    if(valorLexico == NULL) return;

    ListElement* child = valorLexico->children;    
    while (child != NULL){
        printf("%p, %p\n", valorLexico, child->value);
        printDependencies(child->value);
        child = child->next;
    }

    if(valorLexico->next != NULL){
        printf("%p, %p\n", valorLexico, valorLexico->next);
        printDependencies(valorLexico->next);        
    }
}

void printLabels(ValorLexico* valorLexico) {
    if(valorLexico == NULL) return;

    switch (valorLexico->token_type)
    {
        case IDENTIFIER:
        case LITERAL_STRING:
        case SPECIAL_KEYWORD:
            printf("%p [label=\"%s\"];\n", valorLexico, valorLexico->token_value.string);
            break;
        
        case SPECIAL_CHAR:
        case LITERAL_CHAR:
            printf("%p [label=\"%c\"];\n", valorLexico, valorLexico->token_value.character);
            break;

        case LITERAL_BOOL:
            if (valorLexico->token_value.boolean) printf("%p [label=\"true\"];\n", valorLexico);
            else printf("%p [label=\"false\"];\n", valorLexico);
            break;

        case LITERAL_INT:
            printf("%p [label=\"%d\"];\n", valorLexico, valorLexico->token_value.integer);
            break;

        case LITERAL_FLOAT:
            printf("%p [label=\"%f\"];\n", valorLexico, valorLexico->token_value.floating_point);
            break;

        default:
            break;
    }

    ListElement* child = valorLexico->children;    
    while (child != NULL){
        printLabels(child->value);
        child = child->next;
    }

    if(valorLexico->next != NULL) {
        return printLabels(valorLexico->next);
    }
};

ValorLexico* createStringValorLexico(enum TokenType type, char* string){
    TokenValue value = {.string = string}; 

    ValorLexico* functionNode = malloc(sizeof(ValorLexico));
    functionNode->token_type = type;
    functionNode->token_value = value;
    functionNode->children = NULL;
    functionNode->next = NULL;

    return functionNode;
}

ValorLexico* createIntegerValorLexico(int integer){
    TokenValue value = {.integer = integer}; 

    ValorLexico* functionNode = malloc(sizeof(ValorLexico));
    functionNode->token_type = LITERAL_INT;
    functionNode->token_value = value;
    functionNode->children = NULL;
    functionNode->next = NULL;

    return functionNode;
}

ValorLexico* createFloatValorLexico(float floating_point){
    TokenValue value = {.floating_point = floating_point}; 

    ValorLexico* functionNode = malloc(sizeof(ValorLexico));
    functionNode->token_type = LITERAL_FLOAT;
    functionNode->token_value = value;
    functionNode->children = NULL;
    functionNode->next = NULL;

    return functionNode;
}

ValorLexico* createSpecialCharValorLexico(char character){
    TokenValue value = {.character = character}; 

    ValorLexico* functionNode = malloc(sizeof(ValorLexico));
    functionNode->token_type = SPECIAL_CHAR;
    functionNode->token_value = value;
    functionNode->children = NULL;
    functionNode->next = NULL;

    return functionNode;
}

ValorLexico* createCharValorLexico(char character){
    TokenValue value = {.character = character}; 

    ValorLexico* functionNode = malloc(sizeof(ValorLexico));
    functionNode->token_type = LITERAL_CHAR;
    functionNode->token_value = value;
    functionNode->children = NULL;
    functionNode->next = NULL;

    return functionNode;
}

ValorLexico* createBoolValorLexico(bool boolean){
    TokenValue value = {.boolean = boolean}; 

    ValorLexico* functionNode = malloc(sizeof(ValorLexico));
    functionNode->token_type = LITERAL_BOOL;
    functionNode->token_value = value;
    functionNode->children = NULL;
    functionNode->next = NULL;

    return functionNode;
}

ValorLexico* appendToValorLexico(ValorLexico* parent, ValorLexico* child) {
    if (parent == NULL) {
        return child;
    }

    ValorLexico* lastParent = parent;    
    while (lastParent->next != NULL){
        lastParent = lastParent->next;
    }

    lastParent->next = child;
    return parent;
};

ListElement* appendToList(ListElement* parent, ValorLexico* item) {
    if (item == NULL) {
        return parent;
    }

    ListElement* listElement = malloc(sizeof(ListElement));
    listElement->value = item;
    listElement->next = NULL;

    if (parent == NULL) {
        return listElement;
    }
    
    ListElement* lastItem = parent;    
    while (lastItem->next != NULL){
        lastItem = lastItem->next;
    }

    lastItem->next = listElement;
    return parent;
};

void freeValorLexico(ValorLexico* lexical_value) {
    char* string = NULL;
    switch(lexical_value->token_type) {
        case IDENTIFIER:
        case LITERAL_STRING:
            string = lexical_value->token_value.string;
            if(string != NULL) {
                free(string);
            }
            break;
    }
    if(lexical_value->children != NULL) {
        ListElement* children = lexical_value->children;
        lexical_value->children = NULL;
        freeListElement(children);
    }
    if(lexical_value->next != NULL) {
        ValorLexico* next_value = lexical_value->next;
        lexical_value->next = NULL;
        freeValorLexico(next_value);
    }
    free(lexical_value);
}

void freeListElement(ListElement* list_element) {
    if(list_element->value != NULL) {
        ValorLexico* value = list_element->value;
        list_element->value = NULL;
        freeValorLexico(value);
    }
    if(list_element->next != NULL) {
        ListElement* next = list_element->next;
        list_element->next = NULL;
        freeListElement(next);
    }
    free(list_element);
}

