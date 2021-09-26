// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include "abstract_syntax_tree.h"


struct Literal* new_literal(struct ValorLexico valor_lexico) {
    struct Literal* new_pointer = (struct Literal*) malloc(sizeof(struct Literal));
    new_pointer->valor_lexico = valor_lexico;
    return new_pointer;
}

struct Identifier* new_identifier(struct ValorLexico valor_lexico) {
    struct Identifier* new_pointer = (struct Identifier*) malloc(sizeof(struct Identifier));
    new_pointer->valor_lexico = valor_lexico;
    return new_pointer;
}

void print_init_var_nodes(struct CommandList* command_list) {
    struct InitVar init_var = command_list->command_data.init_var;
    printf("\n%p, %p", command_list, init_var.identifier);
    switch(init_var.init_type) {
        case LITERAL_INIT:
            printf("\n%p, %p", command_list, init_var.init_data.literal);
            break;
        case IDENTIFIER_INIT:
            printf("\n%p, %p", command_list, init_var.init_data.identifier);
            break;
        default:
            printf("\nError in print_init_var_nodes -> init_type: %d", init_var.init_type);
    }
}

void print_init_var_label(struct CommandList* command_list) {
    struct InitVar init_var = command_list->command_data.init_var;
    printf("\n%p [label=\"<=\"];", command_list);
    printf("\n%p [label=\"%s\"];", init_var.identifier, init_var.identifier->valor_lexico.token_value.string);
    struct Literal* literal;
    struct Identifier* identifier;
    switch(init_var.init_type) {
        case LITERAL_INIT:
            literal = init_var.init_data.literal;
            struct ValorLexico valor_lexico = literal->valor_lexico;
            switch(valor_lexico.token_type) {
                case LITERAL_INT:
                    printf("\n%p [label=\"%d\"];", literal, valor_lexico.token_value.integer);
                    break;
                case LITERAL_FLOAT:
                    printf("\n%p [label=\"%f\"];", literal, valor_lexico.token_value.floating_point);
                    break;
                case LITERAL_CHAR:
                    printf("\n%p [label=\"%c\"];", literal, valor_lexico.token_value.character);
                    break;
                case LITERAL_BOOL:
                    if(valor_lexico.token_value.boolean) {
                    } else {
                        printf("\n%p [label=\"false\"];", literal);
                    }
                    break;
                case LITERAL_STRING:
                    printf("\n%p [label=\"%s\"];", literal, valor_lexico.token_value.string);
                    break;
            }
            break;
        case IDENTIFIER_INIT:
            identifier = init_var.init_data.identifier;
            printf("\n%p [label=\"%s\"];", identifier, identifier->valor_lexico.token_value.string);
            break;
        default:
            printf("\nError in print_init_var_label -> init_type: %d", init_var.init_type);
    }
}

struct CommandList* new_command(enum CommandType command_type, union CommandData command_data) {
	struct CommandList* new_pointer = (struct CommandList*) malloc(sizeof(struct CommandList));
	new_pointer->command_type = command_type;
	new_pointer->command_data = command_data;
	new_pointer->next_command = NULL;
	return new_pointer;
}

void append_command(struct CommandList* parent, struct CommandList* child) {
	if(parent == NULL || child == NULL) {
		return;
	}
	parent->next_command = child;
}

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

void print_command_nodes(struct CommandList* command_list){
    switch(command_list->command_type) {
        case INIT_VAR:
            print_init_var_nodes(command_list);
            break;
        default:
            printf("\nUnimplemented command_type in print_command_nodes: %d", command_list->command_type);
    }
    struct CommandList* next_command = command_list->next_command;
    while(next_command != NULL) {
        printf("\n%p, %p", command_list, next_command);
        print_command_nodes(next_command);
        next_command = next_command->next_command;
    }
}

void print_command_label(struct CommandList* command_list) {
    switch(command_list->command_type) {
        case INIT_VAR:
            print_init_var_label(command_list);
            break;
        default:
            printf("\nUnimplemented command_type in print_command_label: %d", command_list->command_type);
    }
    struct CommandList* next_command = command_list->next_command;
    while(next_command != NULL) {
        print_command_label(next_command);
        next_command = next_command->next_command;
    }
}

void print_function_nodes(struct FunctionDef* function_def) {
    struct CommandList* current_command = function_def->first_command;
    while(current_command != NULL) {
        printf("\n%p, %p", function_def, current_command);
        print_command_nodes(current_command);
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
    struct CommandList* current_command = function_def->first_command;
    while(current_command != NULL) {
        print_command_label(current_command);
        current_command = current_command->next_command;
    }
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
