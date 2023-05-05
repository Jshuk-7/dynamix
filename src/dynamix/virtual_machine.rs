use crate::{
    byte_block::{ByteBlock, OpCode},
    constant::Constant,
    disassembler::Disassembler,
    stack::Stack,
};

pub enum InterpretResult {
    CompileError,
    RuntimeError,
    Ok,
}

const STACK_STARTING_CAP: usize = 256;

pub struct VirtualMachine {
    block: ByteBlock,
    ip: *const u8,
    origin: *const u8,
    stack: Stack<Constant>,
}

macro_rules! binary_op {
    ($self:expr, $op:tt, $result:expr) => {
        if let Some(rhs) = $self.stack.pop() {
            if let Some(lhs) = $self.stack.pop() {
                if let Constant::Number(x) = lhs {
                    if let Constant::Number(y) = rhs {
                        $self.stack.push(Constant::Number(x $op y))
                    }
                }
            } else {
                $result = InterpretResult::RuntimeError;
                break;
            };
        } else {
            $result = InterpretResult::RuntimeError;
            break;
        }
    };
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            block: ByteBlock::new(),
            ip: std::ptr::null::<u8>(),
            origin: std::ptr::null::<u8>(),
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
                println!();
                Disassembler::disassemble_instruction(&self.block, &mut offset);
            }

            let instruction = if let Some(code) = self.read_byte() {
                code
            } else {
                break;
            };

            match OpCode::from(instruction) {
                Ok(opcode) => match opcode {
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
                    OpCode::Null => self.stack.push(Constant::Null),
                    OpCode::True => self.stack.push(Constant::Bool(true)),
                    OpCode::False => self.stack.push(Constant::Bool(false)),
                    OpCode::Char => {
                        if let Some(constant) = self.read_constant() {
                            self.stack.push(constant);
                        }
                    }
                    OpCode::Negate => {
                        if let Some(constant) = self.stack.pop() {
                            match constant {
                                Constant::Number(x) => self.stack.push(Constant::Number(-x)),
                                _ => {
                                    self.runtime_error("Operand must be a number".to_string());
                                    result = InterpretResult::RuntimeError;
                                    break;
                                }
                            }
                        }
                    }
                    OpCode::Add => binary_op!(self, +, result),
                    OpCode::Sub => binary_op!(self, -, result),
                    OpCode::Mul => binary_op!(self, *, result),
                    OpCode::Div => binary_op!(self, /, result),
                    OpCode::Return => {
                        if let Some(constant) = self.stack.pop() {
                            println!("{constant}");
                        }

                        break;
                    }
                },
                Err(..) => result = InterpretResult::RuntimeError,
            }
        }

        result
    }

    fn runtime_error(&mut self, msg: String) {
        let instruction = self.ip as usize - self.origin as usize;
        let line = self.block.lines[instruction];
        println!("[line:{line:2}] Runtime Error: {msg}");
        self.stack.clear();
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}
