use super::ast_node::AstNode;
use lrpar::{NonStreamingLexer, Span};
use std::ffi::c_void;
use std::ptr::addr_of;

#[derive(Debug)]
pub struct GlobalVarDef {
    is_static: bool,
    var_type: Span,
    var_name: Span,
    next: Option<Box<dyn AstNode>>,
}

impl GlobalVarDef {
    pub fn new(
        is_static: bool,
        var_type: Span,
        var_name: Span,
        next: Option<Box<dyn AstNode>>,
    ) -> GlobalVarDef {
        GlobalVarDef {
            is_static,
            var_type,
            var_name,
            next,
        }
    }
}

impl AstNode for GlobalVarDef {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool) {
        if let Some(next) = &self.next {
            if next.is_tree_member() {
                let next_address = addr_of!(*next) as *const c_void;
                if ripple {
                    println!("{:p}, {:p}", own_address, next_address);
                }
                next.print_dependencies(next_address, false);
            } else {
                next.print_dependencies(own_address, ripple);
            }
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        if let Some(next) = &self.next {
            if next.is_tree_member() {
                let next_address = addr_of!(*next) as *const c_void;
                next.print_labels(lexer, next_address);
            } else {
                next.print_labels(lexer, own_address);
            }
        }
    }
    fn is_tree_member(&self) -> bool {
        false
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        let holder = self.next.take();
        match holder {
            Some(mut node) => {
                node.append_to_next(new_last);
                self.next = Some(node)
            }
            None => self.next = Some(new_last),
        };
    }
}

#[derive(Debug)]
pub struct GlobalVecDef {
    is_static: bool,
    var_type: Span,
    var_name: Span,
    vec_size: Span,
    next: Option<Box<dyn AstNode>>,
}

impl GlobalVecDef {
    pub fn new(
        is_static: bool,
        var_type: Span,
        var_name: Span,
        vec_size: Span,
        next: Option<Box<dyn AstNode>>,
    ) -> GlobalVecDef {
        GlobalVecDef {
            is_static,
            var_type,
            var_name,
            vec_size,
            next,
        }
    }
}

impl AstNode for GlobalVecDef {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool) {
        if let Some(next) = &self.next {
            if next.is_tree_member() {
                let next_address = addr_of!(*next) as *const c_void;
                if ripple {
                    println!("{:p}, {:p}", own_address, next_address);
                }
                next.print_dependencies(next_address, false);
            } else {
                next.print_dependencies(own_address, ripple);
            }
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        if let Some(next) = &self.next {
            if next.is_tree_member() {
                let next_address = addr_of!(*next) as *const c_void;
                next.print_labels(lexer, next_address);
            } else {
                next.print_labels(lexer, own_address);
            }
        }
    }
    fn is_tree_member(&self) -> bool {
        false
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        let holder = self.next.take();
        match holder {
            Some(mut node) => {
                node.append_to_next(new_last);
                self.next = Some(node)
            }
            None => self.next = Some(new_last),
        };
    }
}

#[derive(Debug)]
pub struct FnDef {
    is_static: bool,
    return_type: Span,
    fn_name: Span,
    params: Vec<Parameter>,
    commands: Vec<SimpleCommand>,
    next: Option<Box<dyn AstNode>>,
}

impl FnDef {
    pub fn new(
        is_static: bool,
        return_type: Span,
        fn_name: Span,
        params: Vec<Parameter>,
        commands: Vec<SimpleCommand>,
        next: Option<Box<dyn AstNode>>,
    ) -> FnDef {
        FnDef {
            is_static,
            return_type,
            fn_name,
            params,
            commands,
            next,
        }
    }
}

impl AstNode for FnDef {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        let mut parent = own_address;
        for command in &self.commands {
            if !command.is_tree_member() {
                continue;
            }
            let command_address = addr_of!(command) as *const c_void;
            println!("{:p}, {:p}", parent, command_address);
            command.print_dependencies(command_address, false);
            parent = command_address;
        }
        if let Some(next) = &self.next {
            if next.is_tree_member() {
                let next_address = addr_of!(*next) as *const c_void;
                println!("{:p}, {:p}", own_address, next_address);
                next.print_dependencies(next_address, false);
            } else {
                next.print_dependencies(own_address, true);
            }
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        println!(
            "{:p} [label=\"{}\"];",
            own_address,
            lexer.span_str(self.fn_name)
        );
        for command in &self.commands {
            if !command.is_tree_member() {
                continue;
            }
            command.print_labels(lexer, addr_of!(command) as *const c_void);
        }
        if let Some(next) = &self.next {
            if next.is_tree_member() {
                let next_address = addr_of!(*next) as *const c_void;
                next.print_labels(lexer, next_address);
            } else {
                next.print_labels(lexer, own_address);
            }
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        let holder = self.next.take();
        match holder {
            Some(mut node) => {
                node.append_to_next(new_last);
                self.next = Some(node)
            }
            None => self.next = Some(new_last),
        };
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub is_const: bool,
    pub param_type: Span,
    pub param_name: Span,
}

#[derive(Debug)]
pub enum SimpleCommand {
    VarDef {
        is_static: bool,
        is_const: bool,
        var_type: Span,
        var_name: Span,
    },
    VarDefInitId {
        is_static: bool,
        is_const: bool,
        var_type: Span,
        var_name: Span,
        var_value: Span,
    },
    VarDefInitLit {
        is_static: bool,
        is_const: bool,
        var_type: Span,
        var_name: Span,
        var_value: Literal,
    },
    VarShift {
        var_name: Span,
        shift_type: Span,
        shift_amount: Span,
    },
    VecShift {
        shift_type: Span,
        vec_access: VecAccess,
        shift_amount: Span,
    },
    VarSet {
        var_name: Span,
        new_value: Expression,
    },
    VecSet {
        vec_access: VecAccess,
        new_value: Expression,
    },
    Input {
        var_name: Span,
    },
    OutputId {
        var_name: Span,
    },
    OutputLit {
        lit_value: Literal,
    },
    Continue,
    Break,
    Return {
        ret_value: Expression,
    },
    FnCall(FnCall),
    If {
        condition: Expression,
        consequence: Vec<SimpleCommand>,
    },
    IfElse {
        condition: Expression,
        if_true: Vec<SimpleCommand>,
        if_false: Vec<SimpleCommand>,
    },
    For {
        count_init: Box<SimpleCommand>,
        count_check: Expression,
        count_iter: Box<SimpleCommand>,
        actions: Vec<SimpleCommand>,
    },
    While {
        condition: Expression,
        consequence: Vec<SimpleCommand>,
    },
}

impl AstNode for SimpleCommand {
    #[allow(unused_variables)]
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        match self {
            SimpleCommand::VarDef {
                is_static,
                is_const,
                var_type,
                var_name,
            } => (),
            SimpleCommand::VarDefInitId {
                is_static,
                is_const,
                var_type,
                var_name,
                var_value,
            } => {
                println!("{:p}, {:p}", own_address, addr_of!(*var_name));
                println!("{:p}, {:p}", own_address, addr_of!(*var_value));
            }
            SimpleCommand::VarDefInitLit {
                is_static,
                is_const,
                var_type,
                var_name,
                var_value,
            } => {
                println!("{:p}, {:p}", own_address, addr_of!(*var_name));
                println!("{:p}, {:p}", own_address, addr_of!(*var_value));
            }
            SimpleCommand::VarShift {
                shift_type,
                var_name,
                shift_amount,
            } => {
                println!("{:p}, {:p}", own_address, addr_of!(*var_name));
                println!("{:p}, {:p}", own_address, addr_of!(*shift_amount));
            }
            SimpleCommand::VecShift {
                shift_type,
                vec_access,
                shift_amount,
            } => {
                let vec_address = addr_of!(*vec_access) as *const c_void;
                println!("{:p}, {:p}", own_address, vec_address);
                println!("{:p}, {:p}", own_address, addr_of!(*shift_amount));
                vec_access.print_dependencies(vec_address, false);
            }
            SimpleCommand::VarSet {
                var_name,
                new_value,
            } => {
                let vec_address = addr_of!(*var_name) as *const c_void;
                let val_address = addr_of!(*new_value) as *const c_void;
                println!("{:p}, {:p}", own_address, vec_address);
                println!("{:p}, {:p}", own_address, val_address);
                new_value.print_dependencies(val_address, false);
            }
            SimpleCommand::VecSet {
                vec_access,
                new_value,
            } => {
                let vec_address = addr_of!(*vec_access) as *const c_void;
                let val_address = addr_of!(*new_value) as *const c_void;
                println!("{:p}, {:p}", own_address, vec_address);
                println!("{:p}, {:p}", own_address, val_address);
                vec_access.print_dependencies(vec_address, false);
                new_value.print_dependencies(val_address, false);
            }
            SimpleCommand::Input { var_name } => {
                println!("{:p}, {:p}", own_address, addr_of!(*var_name));
            }
            SimpleCommand::OutputId { var_name } => {
                println!("{:p}, {:p}", own_address, addr_of!(*var_name));
            }
            SimpleCommand::OutputLit { lit_value } => {
                println!("{:p}, {:p}", own_address, addr_of!(*lit_value));
            }
            SimpleCommand::Continue => (),
            SimpleCommand::Break => (),
            SimpleCommand::Return { ret_value } => {
                println!("{:p}, {:p}", own_address, addr_of!(*ret_value));
            }
            SimpleCommand::FnCall(fn_call) => fn_call.print_dependencies(own_address, false),
            SimpleCommand::If {
                condition,
                consequence,
            } => {
                let condition_address = addr_of!(*condition) as *const c_void;
                println!("{:p}, {:p}", own_address, condition_address);
                condition.print_dependencies(condition_address, false);
                let mut previous_pointer = own_address;
                for command in consequence {
                    if !command.is_tree_member() {
                        continue;
                    }
                    let command_address = addr_of!(*command) as *const c_void;
                    println!("{:p}, {:p}", previous_pointer, command_address);
                    command.print_dependencies(command_address, false);
                    previous_pointer = command_address;
                }
            }
            SimpleCommand::IfElse {
                condition,
                if_true,
                if_false,
            } => {
                let condition_address = addr_of!(*condition) as *const c_void;
                println!("{:p}, {:p}", own_address, condition_address);
                condition.print_dependencies(condition_address, false);
                let mut previous_pointer = own_address;
                for command in if_true {
                    if !command.is_tree_member() {
                        continue;
                    }
                    let command_address = addr_of!(*command) as *const c_void;
                    println!("{:p}, {:p}", previous_pointer, command_address);
                    command.print_dependencies(command_address, false);
                    previous_pointer = command_address;
                }
                previous_pointer = own_address;
                for command in if_false {
                    if !command.is_tree_member() {
                        continue;
                    }
                    let command_address = addr_of!(*command) as *const c_void;
                    println!("{:p}, {:p}", previous_pointer, command_address);
                    command.print_dependencies(command_address, false);
                    previous_pointer = command_address;
                }
            }
            SimpleCommand::For {
                count_init,
                count_check,
                count_iter,
                actions,
            } => {
                let init_address = addr_of!(*count_init) as *const c_void;
                let check_address = addr_of!(*count_check) as *const c_void;
                let iter_address = addr_of!(*count_iter) as *const c_void;
                println!("{:p}, {:p}", own_address, init_address);
                println!("{:p}, {:p}", own_address, check_address);
                println!("{:p}, {:p}", own_address, iter_address);

                let mut previous_pointer = own_address;
                for first_command in actions {
                    if !first_command.is_tree_member() {
                        continue;
                    }
                    previous_pointer = addr_of!(*first_command) as *const c_void;
                    println!("{:p}, {:p}", own_address, previous_pointer);
                    break;
                }

                count_init.print_dependencies(init_address, false);
                count_check.print_dependencies(check_address, false);
                count_iter.print_dependencies(iter_address, false);

                let mut first_passed = false;
                for command in actions {
                    if !command.is_tree_member() {
                        continue;
                    }
                    if !first_passed {
                        first_passed = true;
                        command.print_dependencies(previous_pointer, false);
                        continue;
                    }
                    let command_address = addr_of!(*command) as *const c_void;
                    println!("{:p}, {:p}", previous_pointer, command_address);
                    command.print_dependencies(command_address, false);
                    previous_pointer = command_address;
                }
            }
            SimpleCommand::While {
                condition,
                consequence,
            } => {
                let condition_address = addr_of!(*condition) as *const c_void;
                println!("{:p}, {:p}", own_address, condition_address);
                condition.print_dependencies(condition_address, false);
                let mut previous_pointer = own_address;
                for command in consequence {
                    if !command.is_tree_member() {
                        continue;
                    }
                    let command_address = addr_of!(*command) as *const c_void;
                    println!("{:p}, {:p}", previous_pointer, command_address);
                    command.print_dependencies(command_address, false);
                    previous_pointer = command_address;
                }
            }
        }
    }

    #[allow(unused_variables)]
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        match self {
            SimpleCommand::VarDef {
                is_static,
                is_const,
                var_type,
                var_name,
            } => (),
            SimpleCommand::VarDefInitId {
                is_static,
                is_const,
                var_type,
                var_name,
                var_value,
            } => {
                println!("{:p} [label=\"<=\"];", own_address);
                println!(
                    "{:p} [label=\"{}\"];",
                    addr_of!(*var_name),
                    lexer.span_str(*var_name)
                );
                println!(
                    "{:p} [label=\"{}\"];",
                    addr_of!(*var_value),
                    lexer.span_str(*var_value)
                );
            }
            SimpleCommand::VarDefInitLit {
                is_static,
                is_const,
                var_type,
                var_name,
                var_value,
            } => {
                println!("{:p} [label=\"<=\"];", own_address);
                println!(
                    "{:p} [label=\"{}\"];",
                    addr_of!(*var_name),
                    lexer.span_str(*var_name)
                );
                var_value.print_labels(lexer, addr_of!(*var_value) as *const c_void);
            }
            SimpleCommand::VarShift {
                var_name,
                shift_type,
                shift_amount,
            } => {
                println!(
                    "{:p} [label=\"{}\"];",
                    own_address,
                    lexer.span_str(*shift_type)
                );
                println!(
                    "{:p} [label=\"{}\"];",
                    addr_of!(*var_name),
                    lexer.span_str(*var_name)
                );
                println!(
                    "{:p} [label=\"{}\"];",
                    addr_of!(*shift_amount),
                    lexer.span_str(*shift_amount)
                );
            }
            SimpleCommand::VecShift {
                shift_type,
                vec_access,
                shift_amount,
            } => {
                println!(
                    "{:p} [label=\"{}\"];",
                    own_address,
                    lexer.span_str(*shift_type)
                );
                vec_access.print_labels(lexer, addr_of!(*vec_access) as *const c_void);
                println!(
                    "{:p} [label=\"{}\"];",
                    addr_of!(*shift_amount),
                    lexer.span_str(*shift_amount)
                );
            }
            SimpleCommand::VarSet {
                var_name,
                new_value,
            } => {
                println!("{:p} [label=\"=\"];", own_address);
                println!(
                    "{:p} [label=\"{}\"];",
                    addr_of!(*var_name),
                    lexer.span_str(*var_name)
                );
                new_value.print_labels(lexer, addr_of!(*new_value) as *const c_void);
            }
            SimpleCommand::VecSet {
                vec_access,
                new_value,
            } => {
                println!("{:p} [label=\"=\"];", own_address);
                vec_access.print_labels(lexer, addr_of!(*vec_access) as *const c_void);
                new_value.print_labels(lexer, addr_of!(*new_value) as *const c_void);
            }
            SimpleCommand::Input { var_name } => {
                println!("{:p} [label=\"input\"];", own_address);
                println!(
                    "{:p} [label=\"{}\"];",
                    addr_of!(*var_name),
                    lexer.span_str(*var_name)
                );
            }
            SimpleCommand::OutputId { var_name } => {
                println!("{:p} [label=\"output\"];", own_address);
                println!(
                    "{:p} [label=\"{}\"];",
                    addr_of!(*var_name),
                    lexer.span_str(*var_name)
                );
            }
            SimpleCommand::OutputLit { lit_value } => {
                println!("{:p} [label=\"output\"];", own_address);
                lit_value.print_labels(lexer, addr_of!(*lit_value) as *const c_void);
            }
            SimpleCommand::Continue => println!("{:p} [label=\"continue\"];", own_address),
            SimpleCommand::Break => println!("{:p} [label=\"break\"];", own_address),
            SimpleCommand::Return { ret_value } => {
                println!("{:p} [label=\"return\"];", own_address);
                ret_value.print_labels(lexer, addr_of!(*ret_value) as *const c_void);
            }
            SimpleCommand::FnCall(fn_call) => fn_call.print_labels(lexer, own_address),
            SimpleCommand::If {
                condition,
                consequence,
            } => {
                println!("{:p} [label=\"if\"];", own_address);
                condition.print_labels(lexer, addr_of!(*condition) as *const c_void);
                for command in consequence {
                    command.print_labels(lexer, addr_of!(*command) as *const c_void);
                }
            }
            SimpleCommand::IfElse {
                condition,
                if_true,
                if_false,
            } => {
                println!("{:p} [label=\"if\"];", own_address);
                condition.print_labels(lexer, addr_of!(*condition) as *const c_void);
                for command in if_true {
                    command.print_labels(lexer, addr_of!(*command) as *const c_void);
                }
                for command in if_false {
                    command.print_labels(lexer, addr_of!(*command) as *const c_void);
                }
            }
            SimpleCommand::For {
                count_init,
                count_check,
                count_iter,
                actions,
            } => {
                println!("{:p} [label=\"for\"];", own_address);
                count_init.print_labels(lexer, addr_of!(*count_init) as *const c_void);
                count_check.print_labels(lexer, addr_of!(*count_check) as *const c_void);
                count_iter.print_labels(lexer, addr_of!(*count_iter) as *const c_void);
                for command in actions {
                    command.print_labels(lexer, addr_of!(*command) as *const c_void);
                }
            }
            SimpleCommand::While {
                condition,
                consequence,
            } => {
                println!("{:p} [label=\"while\"];", own_address);
                condition.print_labels(lexer, addr_of!(*condition) as *const c_void);
                for command in consequence {
                    command.print_labels(lexer, addr_of!(*command) as *const c_void);
                }
            }
        }
    }
    fn is_tree_member(&self) -> bool {
        match self {
            SimpleCommand::VarDef { .. } => false,
            _ => true,
        }
    }
    fn set_next(&mut self, _new_next: Box<dyn AstNode>) {}
    fn append_to_next(&mut self, _new_last: Box<dyn AstNode>) {}
}

#[derive(Debug)]
pub struct FnCall {
    pub fn_name: Span,
    pub args: Vec<Expression>,
}

impl AstNode for FnCall {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        let mut parent = own_address;
        for expression in &self.args {
            if !expression.is_tree_member() {
                continue;
            }
            let address = addr_of!(*expression) as *const c_void;
            println!("{:p}, {:p}", parent, address);
            expression.print_dependencies(parent, false);
            parent = address;
        }
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print!("{:p} [label=\"call ", own_address);
        println!("{}\"];", lexer.span_str(self.fn_name));
        for expression in &self.args {
            expression.print_labels(lexer, addr_of!(*expression) as *const c_void)
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, _new_next: Box<dyn AstNode>) {}
    fn append_to_next(&mut self, _new_last: Box<dyn AstNode>) {}
}

#[derive(Debug)]
pub struct VecAccess {
    pub name: Box<Span>,
    pub index: Box<Expression>,
}

impl AstNode for VecAccess {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        println!("{:p}, {:p}", own_address, addr_of!(*self.name));
        println!("{:p}, {:p}", own_address, addr_of!(*self.index));
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        println!("{:p} [label=\"[]\"];", own_address);
        println!(
            "{:p} [label=\"{}\"];",
            addr_of!(*self.name),
            lexer.span_str(*self.name)
        );
        self.index
            .print_labels(lexer, addr_of!(*self.index) as *const c_void);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, _new_next: Box<dyn AstNode>) {}
    fn append_to_next(&mut self, _new_last: Box<dyn AstNode>) {}
}

#[derive(Debug)]
pub enum Literal {
    Int(Span),
    Float(Span),
    Bool(Span),
    Char(Span),
    String(Span),
}

impl AstNode for Literal {
    fn print_dependencies(&self, _own_address: *const c_void, _ripple: bool) {}
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        match self {
            Literal::Int(value) => {
                println!("{:p} [label=\"{}\"];", own_address, lexer.span_str(*value))
            }
            Literal::Float(value) => {
                println!("{:p} [label=\"{}\"];", own_address, lexer.span_str(*value))
            }
            Literal::Bool(value) => {
                println!("{:p} [label=\"{}\"];", own_address, lexer.span_str(*value))
            }
            Literal::Char(value) => {
                let text = lexer.span_str(*value);
                println!(
                    "{:p} [label=\"{}\"];",
                    own_address,
                    &text[1..(text.len() - 1)]
                );
            }
            Literal::String(value) => {
                let text = lexer.span_str(*value);
                println!(
                    "{:p} [label=\"{}\"];",
                    own_address,
                    &text[1..(text.len() - 1)]
                );
            }
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, _new_next: Box<dyn AstNode>) {}
    fn append_to_next(&mut self, _new_last: Box<dyn AstNode>) {}
}

#[derive(Debug)]
pub enum Expression {
    Ternary {
        condition: Box<Expression>,
        if_true: Box<Expression>,
        if_false: Box<Expression>,
    },
    Binary {
        op_type: BinaryType,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Unary {
        op_type: UnaryType,
        operand: Box<Expression>,
    },
    VarAccess(Span),
    VecAccess(VecAccess),
    Literal(Literal),
    FnCall(FnCall),
}

impl AstNode for Expression {
    #[allow(unused_variables)]
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        match self {
            Expression::Ternary {
                condition,
                if_true,
                if_false,
            } => {
                for other in [condition, if_true, if_false] {
                    if !other.is_tree_member() {
                        continue;
                    }
                    let other_address = addr_of!(*other) as *const c_void;
                    println!("{:p}, {:p}", own_address, other_address);
                    other.print_dependencies(other_address, false);
                }
            }
            Expression::Binary { op_type, lhs, rhs } => {
                for other in [lhs, rhs] {
                    if !other.is_tree_member() {
                        continue;
                    }
                    let other_address = addr_of!(*other) as *const c_void;
                    println!("{:p}, {:p}", own_address, other_address);
                    other.print_dependencies(other_address, false);
                }
            }
            Expression::Unary { op_type, operand } => {
                let other_address = addr_of!(*operand) as *const c_void;
                println!("{:p}, {:p}", own_address, other_address);
                operand.print_dependencies(other_address, false);
            }
            Expression::VarAccess(span) => (),
            Expression::VecAccess(vec_access) => vec_access.print_dependencies(own_address, false),
            Expression::Literal(literal) => (),
            Expression::FnCall(fn_call) => fn_call.print_dependencies(own_address, false),
        }
    }
    #[allow(unused_variables)]
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        match self {
            Expression::Ternary {
                condition,
                if_true,
                if_false,
            } => {
                println!("{:p} [label=\"?:\"];", own_address);
                condition.print_labels(lexer, addr_of!(*condition) as *const c_void);
                if_true.print_labels(lexer, addr_of!(*if_true) as *const c_void);
                if_false.print_labels(lexer, addr_of!(*if_false) as *const c_void);
            }
            Expression::Binary { op_type, lhs, rhs } => {
                let id = match op_type {
                    BinaryType::BoolOr => "||",
                    BinaryType::BoolAnd => "&&",
                    BinaryType::BitOr => "|",
                    BinaryType::BitXor => "^",
                    BinaryType::BitAnd => "&",
                    BinaryType::Equal => "==",
                    BinaryType::NotEqual => "!=",
                    BinaryType::Lesser => "<",
                    BinaryType::Greater => ">",
                    BinaryType::LesserEqual => "<=",
                    BinaryType::GreaterEqual => ">=",
                    BinaryType::Add => "+",
                    BinaryType::Sub => "-",
                    BinaryType::Mult => "*",
                    BinaryType::Div => "/",
                    BinaryType::Mod => "%",
                };
                println!("{:p} [label=\"{}\"];", own_address, id);
                lhs.print_labels(lexer, addr_of!(*lhs) as *const c_void);
                rhs.print_labels(lexer, addr_of!(*rhs) as *const c_void);
            }
            Expression::Unary { op_type, operand } => {
                let id = match op_type {
                    UnaryType::Positive => "+",
                    UnaryType::Negative => "-",
                    UnaryType::Not => "!",
                    UnaryType::Address => "&",
                    UnaryType::Pointer => "*",
                    UnaryType::Boolean => "?",
                    UnaryType::Hash => "#",
                };
                println!("{:p} [label=\"{}\"];", own_address, id);
                operand.print_labels(lexer, addr_of!(*operand) as *const c_void);
            }
            Expression::VarAccess(span) => {
                println!("{:p} [label=\"{}\"];", own_address, lexer.span_str(*span))
            }
            Expression::VecAccess(vec_access) => vec_access.print_labels(lexer, own_address),
            Expression::Literal(literal) => literal.print_labels(lexer, own_address),
            Expression::FnCall(fn_call) => fn_call.print_labels(lexer, own_address),
        }
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, _new_next: Box<dyn AstNode>) {}
    fn append_to_next(&mut self, _new_last: Box<dyn AstNode>) {}
}

#[derive(Debug)]
pub enum BinaryType {
    BoolOr,
    BoolAnd,
    BitOr,
    BitXor,
    BitAnd,
    Equal,
    NotEqual,
    Lesser,
    Greater,
    LesserEqual,
    GreaterEqual,
    Add,
    Sub,
    Mult,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum UnaryType {
    Positive,
    Negative,
    Not,
    Address,
    Pointer,
    Boolean,
    Hash,
}
