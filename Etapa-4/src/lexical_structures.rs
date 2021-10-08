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
        print_dependencies_own(&self.commands, own_address);
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
        print_dependencies_own(&self.var_name, own_address);
        print_dependencies_own(&self.var_value, own_address);
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
        print_dependencies_own(&self.var_name, own_address);
        print_dependencies_own(&self.var_value, own_address);
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
pub struct VarLeftShift {
    shift_type: Span,
    var_name: Box<dyn AstNode>,
    shift_amount: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VarLeftShift {
    pub fn new(
        shift_type: Span,
        var_name: Box<dyn AstNode>,
        shift_amount: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VarLeftShift {
        VarLeftShift {
            shift_type,
            var_name,
            shift_amount,
            next,
        }
    }
}

impl AstNode for VarLeftShift {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(&self.var_name, own_address);
        print_dependencies_own(&self.shift_amount, own_address);
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
pub struct VarRightShift {
    shift_type: Span,
    var_name: Box<dyn AstNode>,
    shift_amount: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VarRightShift {
    pub fn new(
        shift_type: Span,
        var_name: Box<dyn AstNode>,
        shift_amount: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VarRightShift {
        VarRightShift {
            shift_type,
            var_name,
            shift_amount,
            next,
        }
    }
}

impl AstNode for VarRightShift {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(&self.var_name, own_address);
        print_dependencies_own(&self.shift_amount, own_address);
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
pub struct VecLeftShift {
    shift_type: Span,
    vec_access: Box<dyn AstNode>,
    shift_amount: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecLeftShift {
    pub fn new(
        shift_type: Span,
        vec_access: Box<dyn AstNode>,
        shift_amount: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VecLeftShift {
        VecLeftShift {
            shift_type,
            vec_access,
            shift_amount,
            next,
        }
    }
}

impl AstNode for VecLeftShift {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(&self.vec_access, own_address);
        print_dependencies_own(&self.shift_amount, own_address);
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
pub struct VecRightShift {
    shift_type: Span,
    vec_access: Box<dyn AstNode>,
    shift_amount: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecRightShift {
    pub fn new(
        shift_type: Span,
        vec_access: Box<dyn AstNode>,
        shift_amount: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VecRightShift {
        VecRightShift {
            shift_type,
            vec_access,
            shift_amount,
            next,
        }
    }
}

impl AstNode for VecRightShift {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(&self.vec_access, own_address);
        print_dependencies_own(&self.shift_amount, own_address);
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
        print_dependencies_own(&self.var_name, own_address);
        print_dependencies_own(&self.new_value, own_address);
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
        print_dependencies_own(&self.vec_access, own_address);
        print_dependencies_own(&self.new_value, own_address);
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
        print_dependencies_own(&self.var_name, own_address);
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
    pub fn new(
        op_name: Span,
        var_name: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> OutputId {
        OutputId {
            op_name,
            var_name,
            next,
        }
    }
}

impl AstNode for OutputId {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(&self.var_name, own_address);
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
        print_dependencies_own(&self.lit_value, own_address);
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
        print_dependencies_own(&self.ret_value, own_address);
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

    fn print_label_fn_call(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
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
            Some(args) => {
                print_dependencies_own(&args, own_address);
                print_dependencies_child(&args, own_address);
            },
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
        print_dependencies_own(&self.condition, own_address);
        print_dependencies_own(&self.consequence, own_address);
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
        print_dependencies_own(&self.condition, own_address);
        print_dependencies_own(&self.if_true, own_address);
        print_dependencies_own(&self.if_false, own_address);
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
        print_dependencies_own(&self.count_init, own_address);
        print_dependencies_own(&self.count_check, own_address);
        print_dependencies_own(&self.count_iter, own_address);
        print_dependencies_own(&self.actions, own_address);
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
        print_dependencies_own(&self.condition, own_address);
        print_dependencies_own(&self.consequence, own_address);
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
pub struct Ternary {
    left_span: Span,
    right_span: Span,
    condition: Box<dyn AstNode>,
    if_true: Box<dyn AstNode>,
    if_false: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl Ternary {
    pub fn new(
        left_span: Span,
        right_span: Span,
        condition: Box<dyn AstNode>,
        if_true: Box<dyn AstNode>,
        if_false: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> Ternary {
        Ternary {
            left_span,
            right_span,
            condition,
            if_true,
            if_false,
            next,
        }
    }

    fn print_label_ternary(&self, own_address: *const c_void) {
        println!("{:p} [label=\"?:\"];", own_address);
    }
}

impl AstNode for Ternary {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(&self.condition, own_address);
        print_dependencies_own(&self.if_true, own_address);
        print_dependencies_own(&self.if_false, own_address);
        print_dependencies_child(&self.condition, own_address);
        print_dependencies_child(&self.if_true, own_address);
        print_dependencies_child(&self.if_false, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        self.print_label_ternary(own_address);
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
pub struct Binary {
    op_span: Span,
    op_type: BinaryType,
    lhs: Box<dyn AstNode>,
    rhs: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl Binary {
    pub fn new(
        op_span: Span,
        op_type: BinaryType,
        lhs: Box<dyn AstNode>,
        rhs: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> Binary {
        Binary {
            op_span,
            op_type,
            lhs,
            rhs,
            next,
        }
    }
}

impl AstNode for Binary {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(&self.lhs, own_address);
        print_dependencies_own(&self.rhs, own_address);
        print_dependencies_child(&self.lhs, own_address);
        print_dependencies_child(&self.rhs, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_span, lexer, own_address);
        print_labels_child(&self.lhs, lexer);
        print_labels_child(&self.rhs, lexer);
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
pub struct Unary {
    op_span: Span,
    op_type: UnaryType,
    operand: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl Unary {
    pub fn new(
        op_span: Span,
        op_type: UnaryType,
        operand: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> Unary {
        Unary {
            op_span,
            op_type,
            operand,
            next,
        }
    }
}

impl AstNode for Unary {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(&self.operand, own_address);
        print_dependencies_child(&self.operand, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.op_span, lexer, own_address);
        print_labels_child(&self.operand, lexer);
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
pub struct VarAccess {
    var_name: Span,
    next: Option<Box<dyn AstNode>>,
}

impl VarAccess {
    pub fn new(var_name: Span, next: Option<Box<dyn AstNode>>) -> VarAccess {
        VarAccess { var_name, next }
    }
}

impl AstNode for VarAccess {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.var_name, lexer, own_address);
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
pub struct VecAccess {
    expr_span: Span,
    vec_name: Box<dyn AstNode>,
    vec_index: Box<dyn AstNode>,
    next: Option<Box<dyn AstNode>>,
}

impl VecAccess {
    pub fn new(
        expr_span: Span,
        vec_name: Box<dyn AstNode>,
        vec_index: Box<dyn AstNode>,
        next: Option<Box<dyn AstNode>>,
    ) -> VecAccess {
        VecAccess {
            expr_span,
            vec_name,
            vec_index,
            next,
        }
    }

    fn print_label_vec_access(&self, own_address: *const c_void) {
        println!("{:p} [label=\"[]\"];", own_address);
    }
}

impl AstNode for VecAccess {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_own(&self.vec_name, own_address);
        print_dependencies_own(&self.vec_index, own_address);
        print_dependencies_child(&self.vec_name, own_address);
        print_dependencies_child(&self.vec_index, own_address);
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        self.print_label_vec_access(own_address);
        print_labels_child(&self.vec_name, lexer);
        print_labels_child(&self.vec_index, lexer);
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
pub struct LiteralInt {
    expr_span: Span,
    next: Option<Box<dyn AstNode>>,
}

impl LiteralInt {
    pub fn new(expr_span: Span, next: Option<Box<dyn AstNode>>) -> LiteralInt {
        LiteralInt { expr_span, next }
    }
}

impl AstNode for LiteralInt {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.expr_span, lexer, own_address);
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
pub struct LiteralFloat {
    expr_span: Span,
    next: Option<Box<dyn AstNode>>,
}

impl LiteralFloat {
    pub fn new(expr_span: Span, next: Option<Box<dyn AstNode>>) -> LiteralFloat {
        LiteralFloat { expr_span, next }
    }
}

impl AstNode for LiteralFloat {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.expr_span, lexer, own_address);
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
pub struct LiteralBool {
    expr_span: Span,
    next: Option<Box<dyn AstNode>>,
}

impl LiteralBool {
    pub fn new(expr_span: Span, next: Option<Box<dyn AstNode>>) -> LiteralBool {
        LiteralBool { expr_span, next }
    }
}

impl AstNode for LiteralBool {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        print_label_self(self.expr_span, lexer, own_address);
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
pub struct LiteralChar {
    expr_span: Span,
    next: Option<Box<dyn AstNode>>,
}

impl LiteralChar {
    pub fn new(expr_span: Span, next: Option<Box<dyn AstNode>>) -> LiteralChar {
        LiteralChar { expr_span, next }
    }

    fn print_label_lit_char(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        let text = lexer.span_str(self.expr_span);
        println!(
            "{:p} [label=\"{}\"];",
            own_address,
            &text[1..(text.len() - 1)]
        );
    }
}

impl AstNode for LiteralChar {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        self.print_label_lit_char(lexer, own_address);
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
pub struct LiteralString {
    expr_span: Span,
    next: Option<Box<dyn AstNode>>,
}

impl LiteralString {
    pub fn new(expr_span: Span, next: Option<Box<dyn AstNode>>) -> LiteralString {
        LiteralString { expr_span, next }
    }

    fn print_label_lit_string(
        &self,
        lexer: &dyn NonStreamingLexer<u32>,
        own_address: *const c_void,
    ) {
        let text = lexer.span_str(self.expr_span);
        println!(
            "{:p} [label=\"{}\"];",
            own_address,
            &text[1..(text.len() - 1)]
        );
    }
}

impl AstNode for LiteralString {
    fn print_dependencies(&self, own_address: *const c_void, _ripple: bool) {
        print_dependencies_next(&self.next, own_address);
    }
    fn print_labels(&self, lexer: &dyn NonStreamingLexer<u32>, own_address: *const c_void) {
        self.print_label_lit_string(lexer, own_address);
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

fn print_dependencies_own(child: &Box<dyn AstNode>, own_address: *const c_void) {
    if child.is_tree_member() {
        let child_address = addr_of!(*child) as *const c_void;
        println!("{:p}, {:p}", own_address, child_address);
    }
}

fn print_dependencies_child(child: &Box<dyn AstNode>, own_address: *const c_void) {
    if child.is_tree_member() {
        let child_address = addr_of!(*child) as *const c_void;
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
