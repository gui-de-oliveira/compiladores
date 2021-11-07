use std::collections::HashMap;

use super::error::CompilerError;

#[derive(Clone, Copy, Debug)]
pub struct Label(u32);

impl Label {
    pub fn to_string(&self) -> String {
        format!("L{}", &self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Register {
    Rfp,
    Rsp,
    Rbss,
    R(u32),
}

impl Register {
    pub fn to_string(&self) -> String {
        match &self {
            Register::Rfp => "rfp".to_string(),
            Register::Rsp => "rsp".to_string(),
            Register::Rbss => "rbss".to_string(),
            Register::R(num) => format!("r{}", num),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Operation {
    Load(Register, Register),
    LoadI(i32, Register),
    LoadAI(Register, i32, Register),
    LoadAO(Register, Register, Register),
    StoreAI(Register, Register, i32),
    StoreAO(Register, Register, Register),
    Jump(Register),
    JumpI(Label),
    Cbr(Register, Label, Label),
    CmpLT(Register, Register, Register),
    CmpLE(Register, Register, Register),
    CmpEQ(Register, Register, Register),
    CmpGE(Register, Register, Register),
    CmpGT(Register, Register, Register),
    CmpNE(Register, Register, Register),
    Halt,
    Nop,
    I2i(Register, Register),
    Add(Register, Register, Register),
    AddI(Register, i32, Register),
    Sub(Register, Register, Register),
    SubI(Register, i32, Register),
    Mult(Register, Register, Register),
    MultI(Register, i32, Register),
    Div(Register, Register, Register),
    DivI(Register, i32, Register),
    And(Register, Register),
    Or(Register, Register),
    Not(Register),
}

impl Operation {
    pub fn to_string(&self) -> Result<String, CompilerError> {
        Ok(match &self {
            Operation::Load(reg_a, reg_b) => {
                format!("load {} => {}", reg_a.to_string(), reg_b.to_string())
            }
            Operation::LoadI(addr, reg) => {
                format!("loadI {} => {}", addr, reg.to_string())
            }
            Operation::LoadAI(reg_a, num, reg_b) => {
                format!(
                    "loadAI {}, {} => {}",
                    reg_a.to_string(),
                    num,
                    reg_b.to_string(),
                )
            }
            Operation::LoadAO(reg_a, reg_b, reg_c) => {
                format!(
                    "loadAO {}, {} => {}",
                    reg_a.to_string(),
                    reg_b.to_string(),
                    reg_c.to_string(),
                )
            }
            Operation::StoreAI(reg_a, reg_b, addr) => format!(
                "storeAI {} => {}, {}",
                reg_a.to_string(),
                reg_b.to_string(),
                addr,
            ),
            Operation::StoreAO(reg_a, reg_b, reg_c) => {
                format!(
                    "storeAO {} => {}, {}",
                    reg_a.to_string(),
                    reg_b.to_string(),
                    reg_c.to_string(),
                )
            }
            Operation::Jump(reg) => format!("jump -> {}", reg.to_string()),
            Operation::JumpI(label) => format!("jumpI -> {}", label.to_string()),
            Operation::Cbr(reg, label_a, label_b) => format!(
                "cbr {} -> {}, {}",
                reg.to_string(),
                label_a.to_string(),
                label_b.to_string()
            ),
            Operation::CmpLT(reg_a, reg_b, reg_c) => format!(
                "cmp_LT {}, {} -> {}",
                reg_a.to_string(),
                reg_b.to_string(),
                reg_c.to_string(),
            ),
            Operation::CmpLE(reg_a, reg_b, reg_c) => format!(
                "cmp_LE {}, {} -> {}",
                reg_a.to_string(),
                reg_b.to_string(),
                reg_c.to_string(),
            ),
            Operation::CmpEQ(reg_a, reg_b, reg_c) => format!(
                "cmp_EQ {}, {} -> {}",
                reg_a.to_string(),
                reg_b.to_string(),
                reg_c.to_string(),
            ),
            Operation::CmpGE(reg_a, reg_b, reg_c) => format!(
                "cmp_GE {}, {} -> {}",
                reg_a.to_string(),
                reg_b.to_string(),
                reg_c.to_string(),
            ),
            Operation::CmpGT(reg_a, reg_b, reg_c) => format!(
                "cmp_GT {}, {} -> {}",
                reg_a.to_string(),
                reg_b.to_string(),
                reg_c.to_string(),
            ),
            Operation::CmpNE(reg_a, reg_b, reg_c) => format!(
                "cmp_NE {}, {} -> {}",
                reg_a.to_string(),
                reg_b.to_string(),
                reg_c.to_string(),
            ),
            Operation::Halt => format!("halt"),
            Operation::Nop => format!("nop"),
            Operation::I2i(reg_a, reg_b) => {
                format!("i2i {} => {}", reg_a.to_string(), reg_b.to_string())
            }
            Operation::Add(reg_a, reg_b, reg_c) => {
                format!(
                    "add {}, {} => {}",
                    reg_a.to_string(),
                    reg_b.to_string(),
                    reg_c.to_string(),
                )
            }
            Operation::AddI(reg_a, num, reg_b) => {
                format!(
                    "addI {}, {} => {}",
                    reg_a.to_string(),
                    num,
                    reg_b.to_string(),
                )
            }
            Operation::Sub(reg_a, reg_b, reg_c) => {
                format!(
                    "sub {}, {} => {}",
                    reg_a.to_string(),
                    reg_b.to_string(),
                    reg_c.to_string(),
                )
            }
            Operation::SubI(reg_a, num, reg_b) => {
                format!(
                    "subI {}, {} => {}",
                    reg_a.to_string(),
                    num,
                    reg_b.to_string(),
                )
            }
            Operation::Mult(reg_a, reg_b, reg_c) => {
                format!(
                    "mult {}, {} => {}",
                    reg_a.to_string(),
                    reg_b.to_string(),
                    reg_c.to_string(),
                )
            }
            Operation::MultI(reg_a, num, reg_b) => {
                format!(
                    "multI {}, {} => {}",
                    reg_a.to_string(),
                    num,
                    reg_b.to_string(),
                )
            }
            Operation::Div(reg_a, reg_b, reg_c) => {
                format!(
                    "div {}, {} => {}",
                    reg_a.to_string(),
                    reg_b.to_string(),
                    reg_c.to_string(),
                )
            }
            Operation::DivI(reg_a, num, reg_b) => {
                format!(
                    "divI {}, {} => {}",
                    reg_a.to_string(),
                    num,
                    reg_b.to_string(),
                )
            }
            Operation::And(reg_a, reg_b) => {
                format!("and {} => {}", reg_a.to_string(), reg_b.to_string())
            }
            Operation::Or(reg_a, reg_b) => {
                format!("or {} => {}", reg_a.to_string(), reg_b.to_string())
            }
            Operation::Not(reg) => format!("not -> {}", reg.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Voucher(u32);

#[derive(Clone, Debug)]
pub enum CodeLine {
    Promise(Voucher),
    Deliver(Instruction),
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Unlabeled(Operation),
    Labeled(Label, Operation),
}

impl Instruction {
    pub fn to_string(&self) -> Result<String, CompilerError> {
        match &self {
            Instruction::Unlabeled(operation) => operation.to_string(),
            Instruction::Labeled(label, operation) => {
                Ok(format!("{}: {}", label.to_string(), operation.to_string()?))
            }
        }
    }
}

const SIZE_PROMISE: Voucher = Voucher(0);
const MAIN_PROMISE: Voucher = Voucher(1);

pub struct IlocCode {
    code_lines: Vec<CodeLine>,
    label_map: HashMap<String, Label>,
    label_counter: u32,
    register_counter: u32,
    promise_counter: u32,
    payment_map: HashMap<Voucher, Vec<Instruction>>,
}

impl IlocCode {
    pub fn new() -> IlocCode {
        let starting_register = Register::R(0);
        let register_counter = 1;
        let label_counter = 0;
        let promise_counter = 2;

        let code_lines = vec![
            CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                1024,
                Register::Rfp,
            ))),
            CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                1024,
                Register::Rsp,
            ))),
            CodeLine::Promise(SIZE_PROMISE),
            CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                8,
                starting_register,
            ))),
            CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
                starting_register,
                Register::Rsp,
                0,
            ))),
            CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
                Register::Rsp,
                Register::Rsp,
                4,
            ))),
            CodeLine::Deliver(Instruction::Unlabeled(Operation::StoreAI(
                Register::Rfp,
                Register::Rsp,
                8,
            ))),
            CodeLine::Promise(MAIN_PROMISE),
            CodeLine::Deliver(Instruction::Unlabeled(Operation::Halt)),
        ];

        let label_map = HashMap::new();
        let payment_map = HashMap::new();

        IlocCode {
            code_lines,
            label_map,
            label_counter,
            register_counter,
            promise_counter,
            payment_map,
        }
    }

    pub fn generate_promise(&mut self) -> Voucher {
        let promise_number = self.promise_counter;
        self.promise_counter += 1;
        Voucher(promise_number)
    }

    pub fn pay_promise(&mut self, voucher: Voucher, payment: Vec<Instruction>) {
        self.payment_map.insert(voucher, payment);
    }

    pub fn collect_promises(&mut self) -> Result<(), CompilerError> {
        let mut new_code = vec![];
        for code_line in &self.code_lines {
            match code_line {
                size_promise @ CodeLine::Promise(SIZE_PROMISE) => {
                    new_code.push(size_promise.clone())
                } // leave this promise for last.
                CodeLine::Promise(MAIN_PROMISE) => match self.label_map.get(&"main".to_string()) {
                    Some(promised_label) => new_code.push(CodeLine::Deliver(
                        Instruction::Unlabeled(Operation::JumpI(*promised_label)),
                    )),
                    None => {
                        return Err(CompilerError::IlocErrorUndefinedBehavior(format!(
                            "No main() function found."
                        )))
                    }
                },
                CodeLine::Promise(voucher) => {
                    let payment_vector = self.payment_map.get(voucher).ok_or(
                        CompilerError::IlocErrorUndefinedBehavior(format!(
                            "Unpaid voucher: {:?}",
                            voucher
                        )),
                    )?;
                    for instruction in payment_vector {
                        new_code.push(CodeLine::Deliver(instruction.clone()))
                    }
                }
                good @ CodeLine::Deliver(_) => new_code.push(good.clone()),
            }
        }
        let code_len = new_code.len() as i32;
        let size_promise_position = new_code
            .iter()
            .position(|x| match x {
                CodeLine::Promise(SIZE_PROMISE) => true,
                _ => false,
            })
            .ok_or(CompilerError::SanityError(
                "failed to find SIZE_PROMISE position".to_string(),
            ))?;
        new_code.splice(
            size_promise_position..size_promise_position + 1,
            [CodeLine::Deliver(Instruction::Unlabeled(Operation::LoadI(
                code_len,
                Register::Rbss,
            )))],
        );

        self.code_lines = new_code;
        Ok(())
    }

    pub fn print(&self) {
        for code_line in &self.code_lines {
            match code_line {
                CodeLine::Deliver(instruction) => match instruction.to_string() {
                    Ok(instruction) => println!("{}", instruction),
                    Err(error) => println!("{:?}", error),
                },
                CodeLine::Promise(voucher) => println!("Unpaid promise: {:?}", voucher),
            }
        }
    }

    pub fn new_label(&mut self) -> Label {
        let new_label_value = self.label_counter;
        self.label_counter += 1;
        Label(new_label_value)
    }
    pub fn new_register(&mut self) -> Register {
        let new_register_value = self.register_counter;
        self.register_counter += 1;
        Register::R(new_register_value)
    }

    pub fn add_fn_label(&mut self, fn_name: String) -> Label {
        let new_label = self.new_label();
        self.label_map.insert(fn_name, new_label);
        new_label
    }

    pub fn push_code(&mut self, code: CodeLine) {
        self.code_lines.push(code);
    }
}
