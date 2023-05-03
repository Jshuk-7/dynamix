use crate::byte_block::{ByteBlock, OpCode};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VirtualMachine {
    block: ByteBlock,
    ip: *mut u8,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            block: ByteBlock::new(),
            ip: 0 as *mut u8,
        }
    }

    pub fn interpret(&mut self, block: &ByteBlock) -> InterpretResult {
        self.block = block.clone();
        self.ip = block.bytes.as_ptr() as *mut u8;

        self.run()
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.ip.is_null() {
            return None;
        }

        unsafe {
            let byte = *self.ip;
            self.ip = self.ip.add(1);
            return Some(byte);
        }
    }

    fn run(&mut self) -> InterpretResult {
        while let Some(instruction) = self.read_byte() {
            return match OpCode::from(instruction) {
                Ok(opcode) => match opcode {
                    OpCode::Return => InterpretResult::Ok,
                    OpCode::Constant => InterpretResult::Ok,
                    OpCode::ConstantLong => InterpretResult::Ok,
                },
                Err(..) => InterpretResult::RuntimeError,
            };
        }

        InterpretResult::Ok
    }
}
