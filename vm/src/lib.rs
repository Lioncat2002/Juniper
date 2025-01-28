
pub mod core;


#[cfg(test)]
mod test {
    use crate::core::{
        codegen::builder::Builder, codegen::code::Code, 
        instruction_set::instruction_table, juniper_val::JnpVal, vm::VM,
    };

    #[test]
    pub fn add_program() {
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
        assert_eq!(result.to_number().unwrap(),5);
    }

    #[test]
    pub fn if_program() {
        let it = instruction_table();
        let mut builder = Builder::new(&it);
        let condition = JnpVal::from(0);
        builder.push("push", vec![condition]);

        builder.push("if", vec![JnpVal::from("true_label")]);

        builder.push("push", vec![JnpVal::from("it was false")]);
        builder.push("jmp", vec![JnpVal::from("end")]);

        builder.label("true_label");
        builder.push("push", vec![JnpVal::from("it was true")]);

        builder.label("end");
        println!("If program:\n{:?}", builder);
        let code = Code::from(builder);
        let mut vm = VM::new(code, &it);
        vm.run();
        let result = vm.operand_pop();
        assert_eq!(result.to_str().unwrap(),"it was false");
    }
}
