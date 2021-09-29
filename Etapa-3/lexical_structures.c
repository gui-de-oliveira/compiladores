// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include "lexical_structures.h"

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