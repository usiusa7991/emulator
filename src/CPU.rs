use crate::register::Registers;
use crate::instruction::*;

pub struct CPU {
  pub registers: Registers,
  pub pc: u16,
  pub sp: u16,
  pub bus: MemoryBus,
}

pub struct MemoryBus {
  memory: [u8; 0x10000]
}

impl MemoryBus {
  pub fn new() -> MemoryBus {
    MemoryBus { memory: [0; 0x10000] }
  }

  pub fn read_byte(&self, address: u16) -> u8 {
    self.memory[address as usize]
  }

  pub fn write_byte(&mut self, address: u16, value: u8) {
    self.memory[address as usize] = value;
  }
}

impl CPU {
  pub fn new() -> CPU {
    CPU {
      registers: Registers::new(),
      pc: 0,
      sp: 0,
      bus: MemoryBus::new(),
    }
  }

  fn read_next_byte(&self) -> u8 {
    self.bus.read_byte(self.pc + 1)
  }

  fn call(&mut self, should_jump: bool) -> u16 {
    let next_pc = self.pc.wrapping_add(3);
    if should_jump {
      self.push(next_pc);
      self.jump(true)
    } else {
      next_pc
    }
  }

  fn return_(&mut self, should_jump: bool) -> u16 {
    if should_jump {
      self.pop()
    } else {
      self.pc.wrapping_add(1)
    }
  }

  fn execute(&mut self, instruction: Instruction) -> u16 {
    match instruction {
      Instruction::NOP => self.pc.wrapping_add(1),
      Instruction::ADD(target, source) => {
        let source_value = match source {
          AddSource::BC => self.registers.get_bc()
        };
        match target {
          AddTarget::HL => self.add_to_hl(source_value),
        };
        match source {
          AddSource::BC => self.pc.wrapping_add(1)
        }
      },
      Instruction::JP(test) => {
        let jump_condition = match test {
            JumpTest::NotZero => !self.registers.f.zero,
            JumpTest::NotCarry => !self.registers.f.carry,
            JumpTest::Zero => self.registers.f.zero,
            JumpTest::Carry => self.registers.f.carry,
            JumpTest::Always => true
        };
        self.jump(jump_condition)
      },
      Instruction::LD(load_type) => {
        match load_type {
          LoadType::Byte(target, source) => {
            let source_value = match source {
              LoadByteSource::A => self.registers.a,
              LoadByteSource::D8 => self.read_next_byte(),
              LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
              _ => { panic!("TODO: implement other sources") }
            };
            match target {
              LoadByteTarget::A => self.registers.a = source_value,
              LoadByteTarget::B => self.registers.b = source_value,
              LoadByteTarget::BCI => self.bus.write_byte(self.registers.get_bc(), source_value),
              LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
              _ => { panic!("TODO: implement other targets") }
            };
            match source {
              LoadByteSource::A  => self.pc.wrapping_add(1),
              LoadByteSource::D8  => self.pc.wrapping_add(2),
              _                   => self.pc.wrapping_add(1),
            }
          },
          LoadType::TwoByte(target, source) => {
            let source_value = match source {
              LoadTwoByteSource::D16 => self.read_immediate_16bit(),
              LoadTwoByteSource::SP => self.sp,
              _ => { panic!("TODO: implement other sources") }
            };
            match target {
              LoadTwoByteTarget::BC => self.registers.set_bc(source_value),
              LoadTwoByteTarget::A16 => {
                let address = self.read_immediate_16bit();

                self.bus.write_byte(address, (source_value & 0xFF) as u8);
                self.bus.write_byte(address + 1, (source_value >> 8) as u8);
              },
              _ => { panic!("TODO: implement other targets") }
            };
            match source {
              LoadTwoByteSource::D16 => self.pc.wrapping_add(3),
              LoadTwoByteSource::SP => self.pc.wrapping_add(3),
              _                   => self.pc.wrapping_add(3),
            }
          }
          _ => { panic!("TODO: implement other load types") }
        }
      },
      Instruction::PUSH(target) => {
        let value = match target {
          StackTarget::BC => self.registers.get_bc(),
          _ => { panic!("TODO: support more targets") }
        };
        self.push(value);
        self.pc.wrapping_add(1)
      },
      Instruction::POP(target) => {
        let result = self.pop();
        match target {
            StackTarget::BC => self.registers.set_bc(result),
            _ => { panic!("TODO: support more targets") }
        };
        self.pc.wrapping_add(1)
      },
      Instruction::CALL(test) => {
        let jump_condition = match test {
            JumpTest::NotZero => !self.registers.f.zero,
            _ => { panic!("TODO: support more conditions") }
        };
        self.call(jump_condition)
      },
      Instruction::RET(test) => {
        let jump_condition = match test {
            JumpTest::NotZero => !self.registers.f.zero,
            _ => { panic!("TODO: support more conditions") }
        };
        self.return_(jump_condition)
      },
      Instruction::INC(target) => {
        match target {
          IncDecTarget::B => {
            let new_value = self.inc_8bit(self.registers.b);
            self.registers.b = new_value;
            self.pc.wrapping_add(1)
          },
          IncDecTarget::BC => {
            let value = self.registers.get_bc();
            let new_value = value.wrapping_add(1);
            self.registers.set_bc(new_value);
            self.pc.wrapping_add(1)
          },
          _ => { panic!("TODO: support more targets") }
        }
      },
      Instruction::DEC(target) => {
        match target {
          IncDecTarget::B => {
            let value  = self.registers.b;
            let new_value = self.dec_8bit(value);
            self.registers.b = new_value;
            self.pc.wrapping_add(1)
          },
          _ => { panic!("TODO: support more targets") }
        }
      },
      Instruction::RLCA => {
        let value = self.registers.a;
        let seventh_bit = value >> 7;
        let new_value = (value << 1) | seventh_bit;
        self.registers.a = new_value;
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = seventh_bit != 0;
        self.pc.wrapping_add(1)
      },
      _ => { /* TODO: support more instructions */ self.pc },
    }
  }

  pub fn step(&mut self) {
    let mut instruction_byte = self.bus.read_byte(self.pc);
    let prefixed = instruction_byte == 0xCB;
    if prefixed {
      instruction_byte = self.bus.read_byte(self.pc + 1);
    }

    let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
      self.execute(instruction)
    } else {
      let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
      panic!("Unkown instruction found for: {}", description)
    };

    self.pc = next_pc;
    println!("{}", next_pc);
  }

  fn jump(&self, should_jump: bool) -> u16 {
    if should_jump {
      // Gameboy is little endian so read pc + 2 as most significant bit
      // and pc + 1 as least significant bit
      let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
      let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
      (most_significant_byte << 8) | least_significant_byte
    } else {
      // If we don't jump we need to still move the program
      // counter forward by 3 since the jump instruction is
      // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
      self.pc.wrapping_add(3)
    }
  }

  fn push(&mut self, value: u16) {
    self.sp = self.sp.wrapping_sub(1);
    self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

    self.sp = self.sp.wrapping_sub(1);
    self.bus.write_byte(self.sp, (value & 0xFF) as u8);
  }

  fn pop(&mut self) -> u16 {
    let lsb = self.bus.read_byte(self.sp) as u16;
    self.sp = self.sp.wrapping_add(1);

    let msb = self.bus.read_byte(self.sp) as u16;
    self.sp = self.sp.wrapping_add(1);

    (msb << 8) | lsb
  }

  fn add_to_hl(&mut self, value: u16) {
    let hl_value = self.registers.get_hl();
    let (new_value, did_overflow) = hl_value.overflowing_add(value);
    self.registers.f.subtract = false;
    self.registers.f.carry = did_overflow;
    self.registers.f.half_carry = (value & 0x0FFF) + (hl_value & 0x0FFF) > 0x0FFF;
    self.registers.set_hl(new_value);
  }

  fn inc_8bit(&mut self, value: u8) -> u8 {
    let new_value = value.wrapping_add(1);

    self.registers.f.zero = new_value == 0;
    self.registers.f.subtract = false;
    self.registers.f.half_carry = (value & 0x0F) + 1 > 0x0F;

    new_value
  }

  fn dec_8bit(&mut self, value: u8) -> u8 {
    let new_value = value.wrapping_sub(1);

    self.registers.f.zero = new_value == 0;
    self.registers.f.subtract = true;
    self.registers.f.half_carry = (value & 0x0F) == 0;

    new_value
  }

  fn read_immediate_16bit(&mut self) -> u16 {
    self.bus.read_byte(self.pc + 1) as u16 | (self.bus.read_byte(self.pc + 2) as u16) << 8
  }
}

