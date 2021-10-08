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
        print_dependencies_ripple(&self.next, own_address, ripple)
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_labels_ripple(&self.next, lexer, own_address)
    }
    fn is_tree_member(&self) -> bool {
        false
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
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
        print_dependencies_ripple(&self.next, own_address, ripple)
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_labels_ripple(&self.next, lexer, own_address)
    }
    fn is_tree_member(&self) -> bool {
        false
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct FnDef {
    is_static: bool,
    return_type: Span,
    fn_name: Span,
    params: Vec<Parameter>,
    commands: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl FnDef {
    pub fn new(
        is_static: bool,
        return_type: Span,
        fn_name: Span,
        params: Vec<Parameter>,
        commands: Box<dyn AstNode>,
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
        print_dependencies_child(&self.commands, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.fn_name, lexer, own_address);
        print_labels_child(&self.commands, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub is_const: bool,
    pub param_type: Span,
    pub param_name: Span,
}

#[derive(Debug)]
pub struct LocalVarDef {
    is_static: bool,
    is_const: bool,
    var_type: Span,
    var_name: Span,
    next: Option<Box<dyn AstNode>>,
}

impl LocalVarDef {
    pub fn new(
        is_static: bool,
        is_const: bool,
        var_type: Span,
        var_name: Span,
        next: Option<Box<dyn AstNode>>,
    ) -> LocalVarDef {
        LocalVarDef {
            is_static,
            is_const,
            var_type,
            var_name,
            next,
        }
    }
}

impl AstNode for LocalVarDef {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool) {
        print_dependencies_ripple(&self.next, own_address, ripple)
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_labels_ripple(&self.next, lexer, own_address)
    }
    fn is_tree_member(&self) -> bool {
        false
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct VarDefInitId {
    op_name: Span,
    is_static: bool,
    is_const: bool,
    var_type: Span,
    var_name: Box<dyn AstNode>,
    var_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VarDefInitId {
    pub fn new(
        op_name: Span,
        is_static: bool,
        is_const: bool,
        var_type: Span,
        var_name: Box<dyn AstNode>,
        var_value: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VarDefInitId {
        VarDefInitId {
            op_name,
            is_static,
            is_const,
            var_type,
            var_name,
            var_value,
            next,
        }
    }
}

impl AstNode for VarDefInitId {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.var_name, own_address);
        print_dependencies_child(&self.var_value, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_child(&self.var_name, lexer);
        print_labels_child(&self.var_value, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct VarDefInitLit {
    op_name: Span,
    is_static: bool,
    is_const: bool,
    var_type: Span,
    var_name: Box<dyn AstNode>,
    var_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VarDefInitLit {
    pub fn new(
        op_name: Span,
        is_static: bool,
        is_const: bool,
        var_type: Span,
        var_name: Box<dyn AstNode>,
        var_value: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VarDefInitLit {
        VarDefInitLit {
            op_name,
            is_static,
            is_const,
            var_type,
            var_name,
            var_value,
            next,
        }
    }
}

impl AstNode for VarDefInitLit {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.var_name, own_address);
        print_dependencies_child(&self.var_value, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_child(&self.var_name, lexer);
        print_labels_child(&self.var_value, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct VarShift {
    shift_type: Span,
    var_name: Box<dyn AstNode>,
    shift_amount: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VarShift {
    pub fn new(
        shift_type: Span,
        var_name: Box<dyn AstNode>,
        shift_amount: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VarShift {
        VarShift {
            shift_type,
            var_name,
            shift_amount,
            next,
        }
    }
}

impl AstNode for VarShift {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.var_name, own_address);
        print_dependencies_child(&self.shift_amount, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.shift_type, lexer, own_address);
        print_labels_child(&self.var_name, lexer);
        print_labels_child(&self.shift_amount, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct VecShift {
    shift_type: Span,
    vec_access: Box<dyn AstNode>,
    shift_amount: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecShift {
    pub fn new(
        shift_type: Span,
        vec_access: Box<dyn AstNode>,
        shift_amount: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VecShift {
        VecShift {
            shift_type,
            vec_access,
            shift_amount,
            next,
        }
    }
}

impl AstNode for VecShift {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.vec_access, own_address);
        print_dependencies_child(&self.shift_amount, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.shift_type, lexer, own_address);
        print_labels_child(&self.vec_access, lexer);
        print_labels_child(&self.shift_amount, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct VarSet {
    op_name: Span,
    var_name: Box<dyn AstNode>,
    new_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VarSet {
    pub fn new(
        op_name: Span,
        var_name: Box<dyn AstNode>,
        new_value: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VarSet {
        VarSet {
            op_name,
            var_name,
            new_value,
            next,
        }
    }
}

impl AstNode for VarSet {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.var_name, own_address);
        print_dependencies_child(&self.new_value, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_child(&self.var_name, lexer);
        print_labels_child(&self.new_value, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct VecSet {
    op_name: Span,
    vec_access: Box<dyn AstNode>,
    new_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecSet {
    pub fn new(
        op_name: Span,
        vec_access: Box<dyn AstNode>,
        new_value: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VecSet {
        VecSet {
            op_name,
            vec_access,
            new_value,
            next,
        }
    }
}

impl AstNode for VecSet {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.vec_access, own_address);
        print_dependencies_child(&self.new_value, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_child(&self.vec_access, lexer);
        print_labels_child(&self.new_value, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct Input {
    op_name: Span,
    var_name: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl Input {
    pub fn new(op_name: Span, var_name: Box<dyn AstNode>, next: Option<Box<dyn AstNode>>) -> Input {
        Input {
            op_name,
            var_name,
            next,
        }
    }
}

impl AstNode for Input {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.var_name, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_child(&self.var_name, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct OutputId {
    op_name: Span,
    var_name: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl OutputId {
    pub fn new(op_name: Span, var_name: Box<dyn AstNode>, next: Option<Box<dyn AstNode>>) -> OutputId {
        OutputId {
            op_name,
            var_name,
            next,
        }
    }
}

impl AstNode for OutputId {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.var_name, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_child(&self.var_name, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct OutputLit {
    op_name: Span,
    lit_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl OutputLit {
    pub fn new(
        op_name: Span,
        lit_value: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> OutputLit {
        OutputLit {
            op_name,
            lit_value,
            next,
        }
    }
}

impl AstNode for OutputLit {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.lit_value, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_child(&self.lit_value, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct Continue {
    op_name: Span,
    next: Option<Box<dyn AstNode>>,
}

impl Continue {
    pub fn new(op_name: Span, next: Option<Box<dyn AstNode>>) -> Continue {
        Continue { op_name, next }
    }
}

impl AstNode for Continue {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct Break {
    op_name: Span,
    next: Option<Box<dyn AstNode>>,
}

impl Break {
    pub fn new(op_name: Span, next: Option<Box<dyn AstNode>>) -> Break {
        Break { op_name, next }
    }
}

impl AstNode for Break {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct Return {
    op_name: Span,
    ret_value: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl Return {
    pub fn new(
        op_name: Span,
        ret_value: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> Return {
        Return {
            op_name,
            ret_value,
            next,
        }
    }
}

impl AstNode for Return {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.ret_value, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_child(&self.ret_value, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct FnCall {
    fn_name: Span,
    args: Option<Box<dyn AstNode>>,
    next: Option<Box<dyn AstNode>>,
}

impl FnCall {
    pub fn new(
        fn_name: Span,
        args: Option<Box<dyn AstNode>>,
        next: Option<Box<dyn AstNode>>,
    ) -> FnCall {
        FnCall {
            fn_name,
            args,
            next,
        }
    }

    fn print_label_fn_call(
        &self,
        lexer: &dyn NonStreamingLexer<u32>,
        own_address: *const c_void,
    ) {
        println!(
            "{:p} [label=\"call {}\"];",
            own_address,
            lexer.span_str(self.fn_name)
        );
    }
}

impl AstNode for FnCall {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        match &self.args {
            Some(args) => print_dependencies_child(&args, own_address),
            None => (),
        };
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        self.print_label_fn_call(lexer, own_address);
        match &self.args {
            Some(args) => print_labels_child(&args, lexer),
            None => (),
        };
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct If {
    op_name: Span,
    condition: Box<dyn AstNode>,
    consequence: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl If {
    pub fn new(
        op_name: Span,
        condition: Box<dyn AstNode>,
        consequence: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> If {
        If {
            op_name,
            condition,
            consequence,
            next,
        }
    }
}

impl AstNode for If {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.condition, own_address);
        print_dependencies_child(&self.consequence, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_child(&self.condition, lexer);
        print_labels_child(&self.consequence, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct IfElse {
    op_name: Span,
    condition: Box<dyn AstNode>,
    if_true: Box<dyn AstNode>,
    if_false: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl IfElse {
    pub fn new(
        op_name: Span,
        condition: Box<dyn AstNode>,
        if_true: Box<dyn AstNode>,
        if_false: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> IfElse {
        IfElse {
            op_name,
            condition,
            if_true,
            if_false,
            next,
        }
    }
}

impl AstNode for IfElse {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.condition, own_address);
        print_dependencies_child(&self.if_true, own_address);
        print_dependencies_child(&self.if_false, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_child(&self.condition, lexer);
        print_labels_child(&self.if_true, lexer);
        print_labels_child(&self.if_false, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct For {
    op_name: Span,
    count_init: Box<dyn AstNode>,
    count_check: Box<dyn AstNode>,
    count_iter: Box<dyn AstNode>,
    actions: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl For {
    pub fn new(
        op_name: Span,
        count_init: Box<dyn AstNode>,
        count_check: Box<dyn AstNode>,
        count_iter: Box<dyn AstNode>,
        actions: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> For {
        For {
            op_name,
            count_init,
            count_check,
            count_iter,
            actions,
            next,
        }
    }
}

impl AstNode for For {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.count_init, own_address);
        print_dependencies_child(&self.count_check, own_address);
        print_dependencies_child(&self.count_iter, own_address);
        print_dependencies_child(&self.actions, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_child(&self.count_init, lexer);
        print_labels_child(&self.count_check, lexer);
        print_labels_child(&self.count_iter, lexer);
        print_labels_child(&self.actions, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct While {
    op_name: Span,
    condition: Box<dyn AstNode>,
    consequence: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl While {
    pub fn new(
        op_name: Span,
        condition: Box<dyn AstNode>,
        consequence: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> While {
        While {
            op_name,
            condition,
            consequence,
            next,
        }
    }
}

impl AstNode for While {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_child(&self.condition, own_address);
        print_dependencies_child(&self.consequence, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_name, lexer, own_address);
        print_labels_child(&self.condition, lexer);
        print_labels_child(&self.consequence, lexer);
        print_labels_next(&self.next, lexer, own_address);
    }
    fn is_tree_member(&self) -> bool {
        true
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
}

#[derive(Debug)]
pub struct EmptyBlock {
    op_occurrence: Span,
    next: Option<Box<dyn AstNode>>,
}

impl EmptyBlock {
    pub fn new(op_occurrence: Span, next: Option<Box<dyn AstNode>>) -> EmptyBlock {
        EmptyBlock {
            op_occurrence,
            next,
        }
    }
}

impl AstNode for EmptyBlock {
    fn print_dependencies(&self, own_address: *const c_void, ripple: bool) {
        print_dependencies_ripple(&self.next, own_address, ripple)
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_labels_ripple(&self.next, lexer, own_address)
    }
    fn is_tree_member(&self) -> bool {
        false
    }
    fn set_next(&mut self, new_next: Box<dyn AstNode>) {
        self.next = Some(new_next);
    }
    fn append_to_next(&mut self, new_last: Box<dyn AstNode>) {
        self.next = append_node(&mut self.next, new_last)
    }
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

fn print_dependencies_ripple(
    next_node: &Option<Box<dyn AstNode>>,
    own_address: *const c_void,
    ripple: bool,
) {
    if let Some(next) = &next_node {
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

fn print_labels_ripple(
    next_node: &Option<Box<dyn AstNode>>,
    lexer: &dyn NonStreamingLexer<u32>,
    own_address: *const c_void,
) {
    if let Some(next) = next_node {
        if next.is_tree_member() {
            let next_address = addr_of!(*next) as *const c_void;
            next.print_labels(lexer, next_address);
        } else {
            next.print_labels(lexer, own_address);
        }
    }
}

fn print_dependencies_child(child: &Box<dyn AstNode>, own_address: *const c_void) {
    if child.is_tree_member() {
        let child_address = addr_of!(*child) as *const c_void;
        println!("{:p}, {:p}", own_address, child_address);
        child.print_dependencies(child_address, false);
    } else {
        child.print_dependencies(own_address, true);
    }
}

fn print_dependencies_next(next_node: &Option<Box<dyn AstNode>>, own_address: *const c_void) {
    if let Some(next) = &next_node {
        if next.is_tree_member() {
            let next_address = addr_of!(*next) as *const c_void;
            println!("{:p}, {:p}", own_address, next_address);
            next.print_dependencies(next_address, false);
        } else {
            next.print_dependencies(own_address, true);
        }
    }
}

fn print_label_self(
    self_span: Span,
    lexer: &dyn NonStreamingLexer<u32>,
    own_address: *const c_void,
) {
    println!(
        "{:p} [label=\"{}\"];",
        own_address,
        lexer.span_str(self_span)
    );
}

fn print_labels_child(child: &Box<dyn AstNode>, lexer: &dyn NonStreamingLexer<u32>) {
    child.print_labels(lexer, addr_of!(*child) as *const c_void);
}

fn print_labels_next(
    next_node: &Option<Box<dyn AstNode>>,
    lexer: &dyn NonStreamingLexer<u32>,
    own_address: *const c_void,
) {
    if let Some(next) = &next_node {
        if next.is_tree_member() {
            let next_address = addr_of!(*next) as *const c_void;
            next.print_labels(lexer, next_address);
        } else {
            next.print_labels(lexer, own_address);
        }
    }
}

fn append_node(
    current_next: &mut Option<Box<dyn AstNode>>,
    new_last: Box<dyn AstNode>,
) -> Option<Box<dyn AstNode>> {
    match current_next.take() {
        Some(mut node) => {
            node.append_to_next(new_last);
            Some(node)
        }
        None => Some(new_last),
    }
}