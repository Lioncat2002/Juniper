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

pub fn if_stmt_test(it: &InstructionTable<JnpVal>) -> Code<JnpVal>{
    let mut builder = Builder::new(&it);
    let condition=JnpVal::from(0);
    builder.push("push", vec![condition]);

    builder.push("if", vec![JnpVal::from("true_label")]);

    builder.push("push", vec![JnpVal::from("it was false")]);
    builder.push("jmp", vec![JnpVal::from("end")]);

    builder.label("true_label");
    builder.push("push", vec![JnpVal::from("it was true")]);

    builder.label("end");
    println!("{:?}", builder);
    Code::from(builder)
}

fn main() {
    let it = instruction_table();
    let code = if_stmt_test(&it);

    let mut vm = VM::new(code, &it);
    vm.run();
    let result = vm.operand_pop();
    println!("Result of program: {:?}",result);
}
