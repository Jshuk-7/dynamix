use dynamix::{
    byte_block::{ByteBlock, OpCode},
    constant::Constant,
    virtual_machine::VirtualMachine,
};

fn main() {
    let mut vm = VirtualMachine::new();

    let mut block = ByteBlock::new();

    block.push(OpCode::Constant as u8, 123);
    block.write_constant(Constant::Double(4.0), 123);

    block.push(OpCode::Constant as u8, 123);
    block.write_constant(Constant::Double(2.0), 123);

    block.push(OpCode::Mul as u8, 123);

    block.push(OpCode::Return as u8, 124);

    vm.interpret(&block);
}
