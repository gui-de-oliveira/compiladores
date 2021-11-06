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

pub enum Address {
    Number(i32),
    Label(Label),
    NumLenPromise,
    LabelPromise(String),
}

impl Address {
    pub fn to_string(&self) -> Result<String, CompilerError> {
        match &self {
            Address::Number(num) => Ok(num.to_string()),
            Address::Label(label) => Ok(label.to_string()),
            Address::NumLenPromise => Err(CompilerError::SanityError(format!(
                "to_string() called for NumLenPromise",
            ))),
            Address::LabelPromise(promise) => Err(CompilerError::SanityError(format!(
                "to_string() called for LabelPromise({})",
                promise
            ))),
        }
    }
    pub fn pay_promises(
        &self,
        code_len: i32,
        label_map: &HashMap<String, Label>,
    ) -> Result<Address, CompilerError> {
        match &self {
            Address::Number(num) => Ok(Address::Number(*num)),
            Address::Label(label) => Ok(Address::Label(*label)),
            Address::NumLenPromise => Ok(Address::Number(code_len)),
            Address::LabelPromise(promise) => match label_map.get(promise) {
                Some(promised_label) => Ok(Address::Label(*promised_label)),
                None => Err(CompilerError::SanityError(format!(
                    "No match found for LabelPromise({})",
                    promise
                ))),
            },
        }
    }
}

#[allow(dead_code)]
pub enum Operation {
    Load(Register, Register),
    LoadI(Address, Register),
    LoadAI(Register, i32, Register),
    LoadAO(Register, Register, Register),
    StoreAI(Register, Register, Address),
    Jump(Register),
    JumpI(Address),
    Halt,
    Nop,
    I2i(Register, Register),
    Add(Register, Register, Register),
    AddI(Register, i32, Register),
    Sub(Register, Register, Register),
    SubI(Register, i32, Register),
    Mult(Register, Register, Register),
    MultI(Register, i32, Register),
}

impl Operation {
    pub fn to_string(&self) -> Result<String, CompilerError> {
        Ok(match &self {
            Operation::Load(reg_a, reg_b) => {
                format!("load {} => {}", reg_a.to_string(), reg_b.to_string())
            }
            Operation::LoadI(addr, reg) => {
                format!("loadI {} => {}", addr.to_string()?, reg.to_string())
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
                addr.to_string()?,
            ),
            Operation::Jump(reg) => format!("jump -> {}", reg.to_string()),
            Operation::JumpI(addr) => format!("jumpI -> {}", addr.to_string()?),
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
        })
    }
    pub fn pay_promises(
        &self,
        code_len: i32,
        label_map: &HashMap<String, Label>,
    ) -> Result<Operation, CompilerError> {
        Ok(match &self {
            Operation::Load(reg_a, reg_b) => Operation::Load(*reg_a, *reg_b),
            Operation::LoadI(addr, reg) => {
                Operation::LoadI(addr.pay_promises(code_len, label_map)?, *reg)
            }
            Operation::LoadAI(reg_a, num, reg_b) => Operation::LoadAI(*reg_a, *num, *reg_b),
            Operation::LoadAO(reg_a, reg_b, reg_c) => Operation::LoadAO(*reg_a, *reg_b, *reg_c),
            Operation::StoreAI(reg_a, reg_b, addr) => {
                Operation::StoreAI(*reg_a, *reg_b, addr.pay_promises(code_len, label_map)?)
            }
            Operation::Jump(reg) => Operation::Jump(*reg),
            Operation::JumpI(addr) => Operation::JumpI(addr.pay_promises(code_len, label_map)?),
            Operation::Halt => Operation::Halt,
            Operation::Nop => Operation::Nop,
            Operation::I2i(reg_a, reg_b) => Operation::I2i(*reg_a, *reg_b),
            Operation::Add(reg_a, reg_b, reg_c) => Operation::Add(*reg_a, *reg_b, *reg_c),
            Operation::AddI(reg_a, num, reg_b) => Operation::AddI(*reg_a, *num, *reg_b),
            Operation::Sub(reg_a, reg_b, reg_c) => Operation::Sub(*reg_a, *reg_b, *reg_c),
            Operation::SubI(reg_a, num, reg_b) => Operation::SubI(*reg_a, *num, *reg_b),
            Operation::Mult(reg_a, reg_b, reg_c) => Operation::Mult(*reg_a, *reg_b, *reg_c),
            Operation::MultI(reg_a, num, reg_b) => Operation::MultI(*reg_a, *num, *reg_b),
        })
    }
}

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
    pub fn pay_promises(
        &self,
        code_len: i32,
        label_map: &HashMap<String, Label>,
    ) -> Result<Instruction, CompilerError> {
        Ok(match &self {
            Instruction::Unlabeled(operation) => {
                Instruction::Unlabeled(operation.pay_promises(code_len, label_map)?)
            }
            Instruction::Labeled(label, operation) => {
                Instruction::Labeled(*label, operation.pay_promises(code_len, label_map)?)
            }
        })
    }
}

pub struct IlocCode {
    instructions: Vec<Instruction>,
    label_map: HashMap<String, Label>,
    label_counter: u32,
    register_counter: u32,
}

impl IlocCode {
    pub fn new() -> IlocCode {
        let starting_register = Register::R(0);
        let register_counter = 1;
        let label_counter = 0;

        let instructions = vec![
            Instruction::Unlabeled(Operation::LoadI(Address::Number(1024), Register::Rfp)),
            Instruction::Unlabeled(Operation::LoadI(Address::Number(1024), Register::Rsp)),
            Instruction::Unlabeled(Operation::LoadI(Address::NumLenPromise, Register::Rbss)),
            Instruction::Unlabeled(Operation::LoadI(Address::Number(8), starting_register)),
            Instruction::Unlabeled(Operation::StoreAI(
                starting_register,
                Register::Rsp,
                Address::Number(0),
            )),
            Instruction::Unlabeled(Operation::StoreAI(
                Register::Rsp,
                Register::Rsp,
                Address::Number(4),
            )), // saves rsp
            Instruction::Unlabeled(Operation::StoreAI(
                Register::Rfp,
                Register::Rsp,
                Address::Number(8),
            )), // saves rfp
            Instruction::Unlabeled(Operation::JumpI(Address::LabelPromise("main".to_string()))), // jumps to main
            Instruction::Unlabeled(Operation::Halt),
        ];

        let label_map = HashMap::new();

        IlocCode {
            instructions,
            label_map,
            label_counter,
            register_counter,
        }
    }

    pub fn pay_promises(&mut self) -> Result<(), CompilerError> {
        let mut new_instructions = vec![];
        let code_len = self.instructions.len() as i32;
        for instruction in &self.instructions {
            new_instructions.push(instruction.pay_promises(code_len, &self.label_map)?)
        }
        self.instructions = new_instructions;
        Ok(())
    }

    pub fn print(&self) {
        for instruction in &self.instructions {
            match instruction.to_string() {
                Ok(instruction) => println!("{}", instruction),
                Err(error) => println!("{:?}", error),
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

    pub fn push_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}
