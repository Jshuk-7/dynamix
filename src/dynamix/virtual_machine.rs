use crate::{
    byte_block::{ByteBlock, OpCode},
    constant::Constant,
    disassembler::Disassembler, stack::Stack,
};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VirtualMachine {
    block: ByteBlock,
    ip: *mut u8,
    origin: *const u8,
    stack: Stack<Constant>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            block: ByteBlock::new(),
            ip: 0 as *mut u8,
            origin: 0 as *const u8,
            stack: Stack::new(),
        }
    }

    pub fn interpret(&mut self, block: &ByteBlock) -> InterpretResult {
        self.block = block.clone();
        self.origin = self.block.bytes.as_ptr();
        self.ip = self.origin as *mut u8;

        self.run()
    }

    fn advance_ip(&mut self) -> u8 {
        unsafe {
            let byte = *self.ip;
            self.ip = self.ip.add(1);
            return byte;
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.ip.is_null() {
            return None;
        }

        let byte = self.advance_ip();
        Some(byte)
    }

    fn read_constant(&mut self) -> Option<Constant> {
        if let Some(byte) = self.read_byte() {
            let constant = self.block.constants[byte as usize];
            return Some(constant);
        }

        None
    }

    fn run(&mut self) -> InterpretResult {
        let mut result = InterpretResult::Ok;

        loop {
            let mut offset = unsafe { self.ip.offset_from(self.origin) as usize };

            let instruction = if let Some(code) = self.read_byte() {
                code
            } else {
                break
            };

            if cfg!(debug_assertions) {
                Disassembler::disassemble_instruction(&self.block, &mut offset);
            }

            match OpCode::from(instruction) {
                Ok(opcode) => match opcode {
                    OpCode::Return => break,
                    OpCode::Constant => {
                        if let Some(constant) = self.read_constant() {
                            println!("{constant}");
                        }
                    }
                    OpCode::ConstantLong => {
                        if let Some(constant) = self.read_constant() {
                            println!("{constant}");
                        }
                    }
                },
                Err(..) => result = InterpretResult::RuntimeError,
            }
        }

        result
    }
}
