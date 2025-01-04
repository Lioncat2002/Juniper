use std::fmt;

use super::{super::instruction::InstructionTable, super::table::Table, super::write_once_table::WriteOnceTable};

pub struct Builder<'a, T: 'a + fmt::Debug + PartialEq> {
    pub instruction_table: &'a InstructionTable<T>,
    pub instructions: Vec<usize>,
    pub labels: WriteOnceTable<usize>,
    pub data: Vec<T>,
}

impl<T: fmt::Debug + PartialEq> Builder<'_, T> {
    pub fn new(instruction_table: &'_ InstructionTable<T>) -> Builder<T> {
        let mut labels = WriteOnceTable::new();
        labels.insert("main", 0);

        Builder {
            instruction_table: &instruction_table,
            instructions: vec![],
            labels,
            data: vec![],
        }
    }

    pub fn push(&mut self, name: &str, args: Vec<T>) {
        let inst = self
            .instruction_table
            .by_name(name)
            .expect(&format!("Unable to find instruction with name {:?}", name));

        if args.len() != inst.arity {
            panic!(
                "Instruction {} has arity of {}, but got {} arguments",
                inst.name,
                inst.arity,
                args.len()
            );
        }

        self.instructions.push(inst.opcode);
        self.instructions.push(inst.arity);

        for arg in args {
            let pos = self.push_data(arg);
            self.instructions.push(pos);
        }
    }

    pub fn label(&mut self, name: &str) {
        let idx = self.instructions.len();
        self.labels.insert(name, idx);
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    fn push_data(&mut self, data: T) -> usize {
        let pos = self.data.iter().position(|d| d == &data);
        match pos {
            Some(pos) => pos,
            None => {
                self.data.push(data);
                self.data.len() - 1
            }
        }
    }
}

impl<'a, T: 'a + fmt::Debug + PartialEq> fmt::Debug for Builder<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for i in 0..self.data.len() {
            result.push_str(&format!("@{} = {:?}\n", i, self.data[i]));
        }

        let mut ip = 0;
        let len = self.instructions.len();
        loop {
            for label in self.labels.keys() {
                let idx = *self.labels.get(&label).unwrap();
                if idx == ip {
                    result.push_str(&format!("\n.{}:\n", label));
                }
            }

            if ip == len {
                break;
            }

            let op_code = self.instructions[ip];
            ip += 1;
            let arity = self.instructions[ip];

            let instr = self
                .instruction_table
                .by_opcode(op_code)
                .unwrap_or_else(|| panic!("Unable to find instruction with op code {}", op_code));

            result.push_str(&format!("\t{}", &instr.name));

            for _j in 0..arity {
                ip += 1;
                let const_idx = self.instructions[ip];
                result.push_str(&format!(" @{}", const_idx));
            }
            result.push('\n');

            ip += 1;
        }

        write!(f, "{}", result)
    }
}
