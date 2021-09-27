// Grupo L

// Guilherme de Oliveira (00278301)
// Jean Pierre Comerlatto Darricarrere (00182408)

#include "lexical_structures.h"
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct CommandList;
struct Expression;
struct ExpressionList;

struct Literal {
	struct ValorLexico valor_lexico;
};

struct Literal* new_literal(struct ValorLexico valor_lexico);

struct Identifier {
	struct ValorLexico valor_lexico;
};

struct Identifier* new_identifier(struct ValorLexico valor_lexico);

struct ArrayIndex {
	struct Identifier* identifier;
	struct Expression* expression;
};

struct ArrayIndex* new_array_index(struct Identifier* identifier, struct Expression* expression);

enum StorageAcessType {
	IDENTIFIER_STORAGE,
	ARRAY_INDEX_STORAGE,
};

/// For optional variable or array access.
union StorageAcessData {
	struct Identifier* identifier;
	struct ArrayIndex* array_index;
};

struct StorageAccess {
	enum StorageAcessType storage_type;
	union StorageAcessData storage_data;
};

struct BinaryOp {
	struct ValorLexico operation;
	struct Expression* left_expression;
	struct Expression* right_expression;
};

struct UnaryOp {
	struct ValorLexico operation;
	struct Expression* expression;
};

struct FunctionCall {
	struct Identifier* identifier;
	struct ExpressionList* first_expression;
};

enum ExpressionType {
	LITERAL,
	IDENTIFIER_EXPRESSION,
	ARRAY_INDEX,
	FUNCTION_CALL_EXPR,
	UNARY_OP,
	BINARY_OP,
};

union ExpressionValue {
	struct Literal* literal;
	struct Identifier* identifier;
	struct ArrayIndex* array_index;
	struct FunctionCall function_call;
	struct UnaryOp unary_op;
	struct BinaryOp binary_op;
};

struct Expression {
	enum ExpressionType expression_type;
	union ExpressionValue expression_value;
};

struct ExpressionList {
	struct Expression expression_data;
	struct ExpressionList* next_expression;
};

enum InitVarType {
	LITERAL_INIT,
	IDENTIFIER_INIT,
};

union InitVarData {
	struct Literal* literal;
	struct Identifier* identifier;
};

struct InitVar {
	struct Identifier* identifier;
	enum InitVarType init_type;
	union InitVarData init_data;
};

struct SetVar {
	struct StorageAccess storage_access;
	struct Expression expression;
};

struct InputOutput {
	struct ValorLexico valor_lexico; // Nao esta na especificacao, mas assumo que seja isto o esperado no print.
	struct Expression expression;
};

struct ShiftCommand {
	struct ValorLexico valor_lexico; // << or >>
	struct StorageAccess left_side;
	struct Literal* right_side; // literal int only
};

struct ReturnCommand {
	struct ValorLexico valor_lexico;
	struct Expression expression;
};

struct BreakCommand {
	struct ValorLexico valor_lexico;
};

struct ContinueCommand {
	struct ValorLexico valor_lexico;
};

struct IfCommand {
	struct Expression control_check;
	struct CommandList* true_first_command;
	struct CommandList* else_first_command; // optional
};

struct ForCommand {
	struct InitVar control_init;
	struct Expression control_check;
	struct SetVar control_iter;
	struct CommandList* first_command;
};

struct WhileCommand {
	struct Expression control_check;
	struct CommandList* first_command;
};

enum CommandType {
	INIT_VAR, // init_var
	SET_VAR, // set_var
	IO, // input_output
	FUNCTION_CALL_COMMAND, // function_call
	SHIFT_COMMAND, // shift_command
	RETURN_COMMAND, // return_command
	BREAK_COMMAND, // break_command
	CONTINUE_COMMAND, // continue_command
	IF_COMMAND, // if_command
	FOR_COMMAND, // for_command
	WHILE_COMMAND, // while_command
};

union CommandData {
	struct InitVar init_var;
	struct SetVar set_var;
	struct InputOutput input_output;
	struct FunctionCall function_call;
	struct ShiftCommand shift_command;
	struct ReturnCommand return_command;
	struct BreakCommand break_command;
	struct ContinueCommand continue_command;
	struct IfCommand if_command;
	struct ForCommand for_command;
	struct WhileCommand while_command;
};

struct CommandList {
	enum CommandType command_type;
	union CommandData command_data;
	struct CommandList* next_command;
};

struct CommandList* new_command(enum CommandType command_type, union CommandData command_data);
void append_command(struct CommandList* parent, struct CommandList* child);

struct FunctionDef {
	struct ValorLexico identifier;
	struct CommandList* first_command;
	struct FunctionDef* next_function;
};

struct FunctionDef* new_function_def(struct ValorLexico identifier, struct CommandList* first_command);
void append_function_def(struct FunctionDef* parent, struct FunctionDef* child);
void print_function_nodes(struct FunctionDef* function_def);
void print_function_label(struct FunctionDef* function);

void print_top_function(struct FunctionDef* top_function);

struct CommandList* createSetVar(struct ValorLexico identificador);
