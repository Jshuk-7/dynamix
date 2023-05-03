use crate::byte_block::{ByteBlock, OpCode};

pub struct Disassembler {}

impl Disassembler {
    pub fn disassemble(block: &ByteBlock, name: &str) {
        println!("-- {name} --");

        let mut offset = 0;
        while offset < block.bytes.len() {
            Disassembler::disassemble_instruction(block, &mut offset);
        }
    }

    fn write_block_instruction(block: &ByteBlock, name: &str, offset: &mut usize) {
        let constant = block.bytes[*offset + 1];
        print!("{name:16} {constant:04} ");
        println!("{}", block.constants[constant as usize]);
        *offset += 2;
    }

    fn constant_instruction(block: &ByteBlock, name: &str, offset: &mut usize) {
        Disassembler::write_block_instruction(block, name, offset)
    }
    
    fn constant_long_instruction(block: &ByteBlock, name: &str, offset: &mut usize) {
        Disassembler::write_block_instruction(block, name, offset)
    }

    fn simple_instruction(name: &str, offset: &mut usize) {
        println!("{name}");
        *offset += 1;
    }

    fn disassemble_instruction(block: &ByteBlock, offset: &mut usize) {
        print!("{:04} ", *offset);

        if *offset > 0 && block.lines[*offset] == block.lines[*offset - 1] {
            print!("   | ");
        } else {
            print!("{:04} ", block.lines[*offset]);
        }

        let instruction = block.bytes[*offset];
        match OpCode::from(instruction) {
            Ok(inst) => match inst {
                OpCode::Return => Disassembler::simple_instruction("OP_RETURN", offset),
                OpCode::Constant => {
                    Disassembler::constant_instruction(block, "OP_CONSTANT", offset)
                }
                OpCode::ConstantLong => {
                    Disassembler::constant_long_instruction(block, "OP_CONSTANT_LONG", offset)
                }
            },
            Err(..) => {
                eprintln!("Unknown opcode '{instruction:04}'");
                *offset += 1;
            }
        }
    }
}
