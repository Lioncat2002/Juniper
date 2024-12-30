use core::{
    builder::Builder, code::Code, instruction::InstructionTable,
    instruction_set::instruction_table, juniper_val::JnpVal, vm::VM,
};

mod core;

pub fn build_program(it: &InstructionTable<JnpVal>) -> Code<JnpVal> {
    let mut builder = Builder::new(&it);
    builder.push("push", vec![JnpVal::Number(2)]);
    builder.push("push", vec![JnpVal::Number(3)]);
    builder.push("call", vec![JnpVal::String("func_add".into())]);
    builder.push("ret", vec![]);
    builder.label("func_add");
    builder.push("add", vec![]);
    builder.push("ret", vec![]);
    println!("{:?}", builder);
    Code::from(builder)
}

fn main() {
    let it = instruction_table();
    let code = build_program(&it);

    let mut vm = VM::new(code, &it);
    vm.run();
    let result = vm.operand_pop();
    println!("Result of program: {:?}",result);
}
