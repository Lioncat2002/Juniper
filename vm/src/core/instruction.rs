use std::collections::HashMap;

use super::vm::VM;

pub type InstructionFn<T> = fn(machine: &mut VM<T>, args: &[usize]);

pub struct Instruction<T> {
    pub opcode: usize,
    pub name: String,
    pub arity: usize, //number of operands to pop from stack
    pub fun: InstructionFn<T>,
}

impl<T> Instruction<T> {
    pub fn new(opcode: usize, name: &str, arity: usize, fun: InstructionFn<T>) -> Instruction<T> {
        Instruction {
            opcode,
            name: name.to_string(),
            arity,
            fun,
        }
    }
}

pub struct InstructionTable<T>(HashMap<usize, Instruction<T>>);

impl<T> InstructionTable<T> {
    pub fn new() -> InstructionTable<T> {
        InstructionTable(HashMap::new())
    }

    pub fn by_opcode(&self, opcode: usize) -> Option<&Instruction<T>> {
        self.0.get(&opcode)
    }

    pub fn by_name(&self, name: &str) -> Option<&Instruction<T>> {
        self.0.values().find(|ref inst| inst.name == name)
    }

    pub fn insert(&mut self, inst: Instruction<T>) {
        self.0.insert(inst.opcode, inst);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn symbols(&self) -> Vec<(usize, String)> {
        let mut result = vec![];
        self.0.keys().for_each(|key| {
            let instr = &self.0[key];
            result.push((instr.opcode, instr.name.clone()));
        });
        result.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
        result
    }
}
