use vm::core::{codegen::{builder::Builder, code::Code}, instruction_set::instruction_table, juniper_val::JnpVal, vm::VM};

fn main(){
    let it = instruction_table();
        let mut builder = Builder::new(&it);
        builder.push("push", vec![JnpVal::Number(2)]);
        builder.push("push", vec![JnpVal::Number(3)]);
        builder.push("call", vec![JnpVal::String("func_add".into())]);
        builder.push("ret", vec![]);
        builder.label("func_add");
        builder.push("add", vec![]);
        builder.push("ret", vec![]);
        println!("Add Program:\n{:?}", builder);
        let code = Code::from(builder);
        let mut vm = VM::new(code, &it);
        vm.run();
        let result = vm.operand_pop();
        println!("{:?}",result);
}