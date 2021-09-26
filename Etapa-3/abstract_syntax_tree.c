// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include "abstract_syntax_tree.h"

struct FunctionDef* new_function_def(struct ValorLexico identifier, struct CommandList* first_command){
    struct FunctionDef* new_pointer = (struct FunctionDef*) malloc(sizeof(struct FunctionDef));
    new_pointer->identifier = identifier;
    new_pointer->first_command = first_command;
    new_pointer->next_function = NULL;
    return new_pointer;
}

void append_function_def(struct FunctionDef* parent, struct FunctionDef* child) {
    if(parent == NULL || child == NULL) {
        return;
    }
    parent->next_function = child;
}

void print_commmand_nodes(struct CommandList* command_list){
    ;
}

void print_function_nodes(struct FunctionDef* function_def) {
    struct CommandList* current_command = function_def->first_command;
    while(current_command != NULL) {
        printf("\n%p, %p", function_def, current_command);
        print_commmand_nodes(current_command);
        current_command = current_command->next_command;
    }
    struct FunctionDef* next_function = function_def->next_function;
    while(next_function != NULL) {
        printf("\n%p, %p", function_def, next_function);
        print_function_nodes(next_function);
        next_function = next_function->next_function;
    }
}

void print_function_label(struct FunctionDef* function_def) {
    struct ValorLexico valor_lexico = function_def->identifier;
    char* label;
    if(valor_lexico.token_type != IDENTIFIER) {
        label = "ERRO!!!";
    } else {
        label = valor_lexico.token_value.string;
    }

    printf("\n%p [label=\"%s\"];", function_def, label);
}

void print_top_function(struct FunctionDef* top_function) {
    struct FunctionDef* current_function = top_function;
    while(current_function != NULL) {
        print_function_nodes(current_function);
        current_function = current_function->next_function;
    }
    current_function = top_function;
    while(current_function != NULL) {
        print_function_label(current_function);
        current_function = current_function->next_function;
    }
}
