use super::{
    builder::Builder,
    code::Code,
    instruction::{Instruction, InstructionTable},
    vm::VM,
};

pub fn push(vm: &mut VM<isize>, args: &[usize]) {
    let arg = *vm.get_data(args[0]);
    vm.operand_push(arg);
}

pub fn add(vm: &mut VM<isize>, _: &[usize]) {
    let rhs = vm.operand_pop();
    let lhs = vm.operand_pop();
    vm.operand_push(lhs + rhs);
}

pub fn instruction_table() -> InstructionTable<isize> {
    let mut it = InstructionTable::new();
    it.insert(Instruction::new(0, "push", 1, push));
    it.insert(Instruction::new(1, "add", 0, add));
    it
}
