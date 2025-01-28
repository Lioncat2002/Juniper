use super::{
    instruction::{Instruction, InstructionTable},
    juniper_val::JnpVal,
    vm::VM,
};

fn nop(_: &mut VM<JnpVal>, _: &[usize]) {}

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

fn jump(vm: &mut VM<JnpVal>, args: &[usize]) {
    let label = vm.get_data(args[0]).clone();
    vm.jump(label.to_str().unwrap());
}

fn if_stmt(vm: &mut VM<JnpVal>, args: &[usize]) {
    let condition = vm.operand_pop().to_number().unwrap();
    //jump to label if condition!=0; i.e. condition=true i.e condition=1
    if condition != 0 {
        let label = vm.get_data(args[0]).clone();
        vm.jump(label.to_str().unwrap());
    }
}

pub fn instruction_table() -> InstructionTable<JnpVal> {
    let mut it = InstructionTable::new();
    it.insert(Instruction::new(0, "nop", 0, nop)); //not really needed but might need in the future?
    it.insert(Instruction::new(1, "push", 1, push));
    it.insert(Instruction::new(2, "add", 0, add));
    it.insert(Instruction::new(3, "call", 1, call));
    it.insert(Instruction::new(4, "ret", 0, ret));
    it.insert(Instruction::new(5, "jmp", 1, jump));
    it.insert(Instruction::new(6, "if", 1, if_stmt));

    it
}
