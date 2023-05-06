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

    fn simple_instruction(name: &str, offset: &mut usize) {
        println!("{name}");
        *offset += 1;
    }

    pub fn disassemble_instruction(block: &ByteBlock, offset: &mut usize) {
        print!("{:04} ", *offset);

        let in_bounds = *offset < block.bytes.len();
        if !in_bounds {
            return;
        }

        let same_line = || {
            if *offset == 0 {
                return false;
            }

            block.lines[*offset] == block.lines[*offset - 1]
        };

        if same_line() {
            print!("   | ");
        } else {
            print!("{:04} ", block.lines[*offset]);
        }

        let instruction = block.bytes[*offset];
        match OpCode::from(instruction) {
            Ok(inst) => match inst {
                OpCode::Print => Disassembler::simple_instruction("OP_PRINT", offset),
                OpCode::Pop => Disassembler::simple_instruction("OP_POP", offset),
                OpCode::DefineGlobal => {
                    Disassembler::simple_instruction("OP_DEFINE_GLOBAL", offset)
                }
                OpCode::GetGlobal => Disassembler::simple_instruction("OP_GET_GLOBAL", offset),
                OpCode::SetGlobal => Disassembler::simple_instruction("OP_SET_GLOBAL", offset),
                OpCode::GetLocal => Disassembler::simple_instruction("OP_GET_LOCAL", offset),
                OpCode::SetLocal => Disassembler::simple_instruction("OP_SET_LOCAL", offset),
                OpCode::JumpIfFalse => Disassembler::simple_instruction("OP_JUMP_IF_FALSE", offset),
                OpCode::Jmp => Disassembler::simple_instruction("OP_JUMP", offset),
                OpCode::Constant => {
                    Disassembler::constant_instruction(block, "OP_CONSTANT", offset)
                }
                OpCode::True => Disassembler::simple_instruction("OP_TRUE", offset),
                OpCode::False => Disassembler::simple_instruction("OP_FALSE", offset),
                OpCode::Char => Disassembler::simple_instruction("OP_CHAR", offset),
                OpCode::Null => Disassembler::simple_instruction("OP_NULL", offset),
                OpCode::Equal => Disassembler::simple_instruction("OP_EQUAL", offset),
                OpCode::Greater => Disassembler::simple_instruction("OP_GREATER", offset),
                OpCode::Less => Disassembler::simple_instruction("OP_LESS", offset),
                OpCode::Negate => Disassembler::simple_instruction("OP_NEGATE", offset),
                OpCode::Not => Disassembler::simple_instruction("OP_NOT", offset),
                OpCode::Add => Disassembler::simple_instruction("OP_ADD", offset),
                OpCode::Sub => Disassembler::simple_instruction("OP_SUB", offset),
                OpCode::Mul => Disassembler::simple_instruction("OP_MUL", offset),
                OpCode::Div => Disassembler::simple_instruction("OP_DIV", offset),
                OpCode::Return => Disassembler::simple_instruction("OP_RETURN", offset),
            },
            Err(..) => {
                eprintln!("Unknown opcode '{instruction:04}'");
                *offset += 1;
            }
        }
    }
}
