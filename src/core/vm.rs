use super::{code::Code, instruction::InstructionTable, stack::Stack};

pub struct VM<'a, T> {
    pub code: Code<T>,
    pub instruction_table: &'a InstructionTable<T>,
    pub ip: usize, //instruction pointer
    pub operand_stack: Stack<T>,
}

impl<'a, T> VM<'a, T> {
    pub fn new(code: Code<T>, instruction_table: &'a InstructionTable<T>) -> VM<'a, T> {
        VM {
            code,
            instruction_table,
            ip: 0,
            operand_stack: Stack::new(),
        }
    }

    pub fn operand_push(&mut self, value: T) {
        self.operand_stack.push(value);
    }

    pub fn operand_pop(&mut self) -> T {
        self.operand_stack.pop()
    }

    pub fn get_data(&self, idx: usize) -> &T {
        self.code
            .data
            .get(idx)
            .expect(&format!("Constant data is not present at index {idx}"))
    }

    pub fn next_code(&mut self) -> usize {
        let code = self.code.code[self.ip];
        self.ip = self.ip + 1;
        code
    }

    pub fn run(&mut self) {
        loop {
            if self.ip == self.code.code.len() {
                break;
            }

            let opcode = self.next_code();
            let arity = self.next_code();

            let inst = self
                .instruction_table
                .by_opcode(opcode)
                .expect(&format!("unable to find instruction with opcode {opcode}"));

            let mut args = vec![];

            for _ in 0..arity {
                args.push(self.next_code());
            }

            let fun = inst.fun;

            fun(self, &args);
        }
    }
}
