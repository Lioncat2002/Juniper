# Juniper
A stack based vm for the [Helix compiler](https://github.com/Lioncat2002/HelixLang)

Still WIP but in the future this is serve as a backend for Helix so that Helix can be embedded as a scripting language

## Example textual representation:
```
@0 = 123
@1 = 456

.main:
  push @0
  push @1
  add
```
`Note: As of now, text format compilation is not implemented yet. To generate code, use the builder API`
## Usage:
You can define your own functions with the signature `fn func_name(vm: &mut VM<T>, args: &[usize])`
```rs
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
```
