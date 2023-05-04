use crate::{
    byte_block::{ByteBlock, OpCode},
    constant::Constant,
    disassembler::Disassembler,
    stack::Stack,
};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

const STACK_STARTING_CAP: usize = 256;

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
            ip: std::ptr::null_mut::<u8>(),
            origin: std::ptr::null_mut::<u8>(),
            stack: Stack::new(STACK_STARTING_CAP),
        }
    }

    pub fn stack(self) -> Stack<Constant> {
        self.stack
    }

    pub fn interpret(&mut self, block: &ByteBlock) -> InterpretResult {
        self.block = block.clone();
        self.origin = self.block.bytes.as_ptr();
        self.ip = self.origin as *mut u8;

        self.run()
    }

    fn advance_ip(&mut self) -> u8 {
        let byte: u8;
        unsafe {
            byte = *self.ip;
            self.ip = self.ip.add(1);
            byte
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

            if cfg!(debug_assertions) && cfg!(feature = "stack-trace") {
                print!("{:10}", ' ');
                let mut slot = self.stack.as_ptr();
                let top = self.stack.top_as_ptr();
                while (slot as usize) < top as usize {
                    unsafe {
                        print!("[ {} ]", *slot);
                        slot = slot.add(1);
                    }
                }
                println!("");
                Disassembler::disassemble_instruction(&self.block, &mut offset);
            }

            let instruction = if let Some(code) = self.read_byte() {
                code
            } else {
                break;
            };

            match OpCode::from(instruction) {
                Ok(opcode) => match opcode {
                    OpCode::Return => {
                        if let Some(constant) = self.stack.pop() {
                            println!("{constant}");
                        }

                        break;
                    }
                    OpCode::Constant => {
                        // remember OP_CONSTANT instruction 'loads' a constant onto the stack
                        if let Some(constant) = self.read_constant() {
                            self.stack.push(constant);
                        }
                    }
                    OpCode::ConstantLong => {
                        if let Some(constant) = self.read_constant() {
                            self.stack.push(constant);
                        }
                    }
                },
                Err(..) => result = InterpretResult::RuntimeError,
            }
        }

        result
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}
