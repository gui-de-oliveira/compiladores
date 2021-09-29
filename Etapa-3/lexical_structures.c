// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include "lexical_structures.h"

void printDependencies(ValorLexico* valorLexico){
    if(valorLexico == NULL) return;

    ListElement* child = valorLexico->children;    
    while (child != NULL){
        printf("%p, %p\n", valorLexico, child);
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

    printf("%p [label=\"%s\"];\n", valorLexico, valorLexico->token_value.string);

    if(valorLexico->next != NULL) {
        return printLabels(valorLexico->next);
    }
};

ValorLexico* createFunction(char* identifier) {
    TokenValue value = {.string = identifier}; 

    ValorLexico* functionNode = malloc(sizeof(ValorLexico));
    functionNode->token_type = IDENTIFIER;
    functionNode->token_value = value;
    functionNode->children = NULL;
    functionNode->next = NULL;
	
    return functionNode;
};

ValorLexico* addAsNext(ValorLexico* parent, ValorLexico* child) {
    ValorLexico* lastParent = parent;    
    while (lastParent->next != NULL){
        lastParent = lastParent->next;
    }

    lastParent->next = child;
    return parent;
};