use super::{
    builder::Builder,
    code::Code,
    instruction::{Instruction, InstructionTable},
    juniper_val::JnpVal,
    vm::VM,
};

fn push(vm: &mut VM<JnpVal>, args: &[usize]) {
    let arg = vm.get_data(args[0]).clone();
    vm.operand_push(arg);
}

fn add(vm: &mut VM<JnpVal>, _: &[usize]) {
    let rhs = vm.operand_pop().to_number().unwrap();
    let lhs = vm.operand_pop().to_number().unwrap();
    vm.operand_push(JnpVal::Number(lhs + rhs));
}

fn call(vm: &mut VM<JnpVal>, args: &[usize]) {
    let label = vm.get_data(args[0]).clone();
    vm.call(label.to_str().unwrap());
}

fn ret(vm: &mut VM<JnpVal>, _: &[usize]) {
    vm.ret();
}

pub fn instruction_table() -> InstructionTable<JnpVal> {
    let mut it = InstructionTable::new();
    it.insert(Instruction::new(0, "push", 1, push));
    it.insert(Instruction::new(1, "add", 0, add));
    it.insert(Instruction::new(2, "call", 1, call));
    it.insert(Instruction::new(3, "ret", 0, ret));

    it
}
