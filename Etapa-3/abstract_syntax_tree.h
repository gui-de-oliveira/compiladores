// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include "lexical_structures.h"
#include <stddef.h>
#include <stdio.h>

struct CommandList;
struct Expression;
struct ExpressionList;

typedef struct Literal {
	valor_lexico_t valor_lexico;
} literal_t;

typedef struct Identifier {
	valor_lexico_t valor_lexico;
} identifier_t;

struct ArrayIndex {
	identifier_t identifier;
	struct ExpressionList* expression;
};

struct ArrayIndex new_array_index(identifier_t identifier, struct Expression expression);

typedef enum StorageAcessType {
	IDENTIFIER_STORAGE,
	ARRAY_INDEX_STORAGE,
} storage_access_type_t;

/// For optional variable or array access.
typedef union StorageAcessData {
	identifier_t identifier;
	struct ArrayIndex array_index;
} storage_access_data_t;

typedef struct StorageAccess {
	storage_access_type_t storage_type;
	storage_access_data_t storage_data;
} storage_access_t;

typedef enum ExpressionType {
	LITERAL,
	IDENTIFIER_EXPRESSION,
	ARRAY_INDEX,
	FUNCTION_CALL_EXPR,
	UNARY_OP,
	BINARY_OP,
} expression_type_t;

struct BinaryOp {
	valor_lexico_t operation;
	struct Expression* left_expression;
	struct Expression* right_expression;
};

struct UnaryOp {
	valor_lexico_t operation;
	struct Expression* expression;
};

struct FunctionCall {
	identifier_t identifier;
	struct ExpressionList* first_expression;
};

union ExpressionValue {
	identifier_t identifier;
	literal_t literal;
	struct ArrayIndex array_index;
	struct FunctionCall function_call;
	struct UnaryOp unary_op;
	struct BinaryOp binary_op;
};

struct Expression {
	expression_type_t expression_type;
	union ExpressionValue expression_value;
};

struct ExpressionList {
	struct Expression expression_data;
	struct ExpressionList* next_expression;
};

typedef struct InitVar {
	identifier_t identifier;
	struct Expression expression;
} init_var_t;

typedef struct SetVar {
	storage_access_t storage_access;
	struct Expression expression;
} set_var_t;

typedef struct InputOutput {
	valor_lexico_t valor_lexico; // Nao esta na especificacao, mas assumo que seja isto o esperado no print.
	struct Expression expression;
} input_output_t;

typedef struct ShiftLeft {
	valor_lexico_t valor_lexico;
	storage_access_t storage_access;
	literal_t literal; // int only
} shift_left_t;

typedef struct ShiftRight {
	valor_lexico_t valor_lexico; // << or >>
	identifier_t identifier;
	storage_access_t storage_access;
	literal_t literal; // int only
} shift_right_t;

typedef struct ReturnCommand {
	valor_lexico_t valor_lexico;
	struct Expression expression;
} return_command_t;

typedef struct BreakCommand {
	valor_lexico_t valor_lexico;
} break_command_t;

typedef struct ContinueCommand {
	valor_lexico_t valor_lexico;
} continue_command_t;

typedef struct IfCommand {
	struct Expression control_check;
	struct CommandList* true_first_command;
	struct CommandList* else_first_command; // optional
} if_command_t;

typedef struct ForCommand {
	init_var_t control_init;
	struct Expression control_check;
	set_var_t control_iter;
	struct CommandList* first_command;
} for_command_t;

typedef struct WhileCommand {
	struct Expression control_check;
	struct CommandList* first_command;
} while_command_t;

typedef enum CommandType {
	INIT_VAR, // init_var
	SET_VAR, // set_var
	IO, // input_output
	FUNCTION_CALL_COMMAND, // function_call
	SHIFT_LEFT, // shift_left
	SHIFT_RIGHT, // shift_right
	RETURN_COMMAND, // return_command
	BREAK_COMMAND, // break_command
	CONTINUE_COMMAND, // continue_command
	IF_COMMAND, // if_command
	FOR_COMMAND, // for_command
	WHILE_COMMAND, // while_command
} command_type_t;

typedef union CommandData {
	init_var_t init_var;
	set_var_t set_var;
	input_output_t input_output;
	struct FunctionCall function_call;
	shift_left_t shift_left;
	shift_right_t shift_right;
	return_command_t return_command;
	break_command_t break_command;
	continue_command_t continue_command;
	if_command_t if_command;
	for_command_t for_command;
	while_command_t while_command;
} command_data_t;

struct CommandList {
	command_type_t command_type;
	command_data_t command_data;
	struct CommandList* next_command;
};

typedef struct FunctionDef {
	valor_lexico_t identifier;
	struct CommandList* first_command;
	struct FunctionDef* next_function;
} function_def_t;
