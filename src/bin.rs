use dynamix::{
    byte_block::{ByteBlock, OpCode},
    constant::Constant,
    virtual_machine::VirtualMachine,
};

fn main() {
    let mut vm = VirtualMachine::new();

    let mut block = ByteBlock::new();

    block.push(OpCode::Constant as u8, 123);
    block.write_constant(Constant::Double(1.3), 123);
    block.push(OpCode::ConstantLong as u8, 123);
    block.write_constant(Constant::Double(16_738.34), 124);
    block.push(OpCode::Return as u8, 125);

    vm.interpret(&block);
}
