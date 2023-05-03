use dynamix::{
    byte_block::{ByteBlock, OpCode},
    disassembler::Disassembler, constant::Constant, virtual_machine::VirtualMachine,
};

fn main() {
    let mut vm = VirtualMachine::new();

    let mut block = ByteBlock::new();
    
    let constant = block.push_constant(Constant::Double(1.3));
    let constant2 = block.push_constant(Constant::Double(54.4));
    
    block.push(OpCode::Constant as u8, 123);
    block.push(constant, 123);
    block.push(OpCode::ConstantLong as u8, 123);
    block.push(constant2, 124);
    block.push(OpCode::Return as u8, 125);

    vm.interpret(&block);

    Disassembler::disassemble(&block, "test block");
}
