use core::{
    builder::Builder, code::Code, instruction::InstructionTable,
    instruction_set::instruction_table, vm::VM,
};

mod core;

pub fn build_program(it: &InstructionTable<isize>) -> Code<isize> {
    let mut builder = Builder::new(&it);
    builder.push("push", vec![2]);
    builder.push("push", vec![3]);
    builder.push("add", vec![]);

    Code::from(builder)
}

fn main() {
    let it = instruction_table();
    let code = build_program(&it);

    let mut vm = VM::new(code, &it);
    vm.run();
    let result = vm.operand_pop();
    println!("Result of program: {result}");
}
