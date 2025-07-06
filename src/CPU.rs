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

  pub fn execute(&mut self, instruction: Instruction) -> u16 {
    match instruction {
      Instruction::NOP => self.pc.wrapping_add(1),
      Instruction::ADD(target, source) => {
        let source_value = match source {
          AddSource::BC => self.registers.get_bc(),
          AddSource::DE => self.registers.get_de(),
          AddSource::HL => self.registers.get_hl()
        };
        match target {
          AddTarget::HL => self.add_to_hl(source_value),
        };
        match source {
          _ => self.pc.wrapping_add(1)
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
      Instruction::JR(conditions) => {
        let skip_counts = self.read_next_byte() as i8;
        let condition_flag = match conditions {
          JumpRelativeConditions::Always => {
            true
          },
          JumpRelativeConditions::NoZeroFlag => {
            self.registers.f.zero == false
          },
          JumpRelativeConditions::ZeroFlag => {
            self.registers.f.zero == true
          },
        };

        if condition_flag {
          (((self.pc.wrapping_add(2)) as i32).wrapping_add(skip_counts as i32)) as u16
        } else {
          self.pc.wrapping_add(2)
        }
      },
      Instruction::LD(load_type) => {
        match load_type {
          LoadType::Byte(target, source) => {
            let source_value = match source {
              LoadByteSource::A => self.registers.a,
              LoadByteSource::D8 => self.read_next_byte(),
              LoadByteSource::BCI => self.bus.read_byte(self.registers.get_bc()),
              LoadByteSource::DEI => self.bus.read_byte(self.registers.get_de()),
              LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
              LoadByteSource::HLP => {
                let hl_value = self.registers.get_hl();
                self.registers.set_hl(hl_value.wrapping_add(1));
                self.bus.read_byte(hl_value)
              },

              _ => { panic!("TODO: implement other sources") }
            };
            match target {
              LoadByteTarget::A => self.registers.a = source_value,
              LoadByteTarget::B => self.registers.b = source_value,
              LoadByteTarget::C => self.registers.c = source_value,
              LoadByteTarget::D => self.registers.d = source_value,
              LoadByteTarget::E => self.registers.e = source_value,
              LoadByteTarget::H => self.registers.h = source_value,
              LoadByteTarget::BCI => self.bus.write_byte(self.registers.get_bc(), source_value),
              LoadByteTarget::DEI => self.bus.write_byte(self.registers.get_de(), source_value),
              LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
              LoadByteTarget::HLP => {
                let hl_value = self.registers.get_hl();
                self.bus.write_byte(self.registers.get_hl(), source_value);
                self.registers.set_hl(hl_value.wrapping_add(1))
              },
              _ => { panic!("TODO: implement other targets") }
            };
            match source {
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
              LoadTwoByteTarget::DE => self.registers.set_de(source_value),
              LoadTwoByteTarget::HL => self.registers.set_hl(source_value),
              LoadTwoByteTarget::A16 => {
                let address = self.read_immediate_16bit();

                self.bus.write_byte(address, (source_value & 0xFF) as u8);
                self.bus.write_byte(address + 1, (source_value >> 8) as u8);
              },
              _ => { panic!("TODO: implement other targets") }
            };
            self.pc.wrapping_add(3)
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
          },
          IncDecTarget::C => {
            let new_value = self.inc_8bit(self.registers.c);
            self.registers.c = new_value;
          },
          IncDecTarget::D => {
            let new_value = self.inc_8bit(self.registers.d);
            self.registers.d = new_value;
          },
          IncDecTarget::E => {
            let new_value = self.inc_8bit(self.registers.e);
            self.registers.e = new_value;
          },
          IncDecTarget::H => {
            let new_value = self.inc_8bit(self.registers.h);
            self.registers.h = new_value;
          },
          IncDecTarget::BC => {
            let value = self.registers.get_bc();
            let new_value = value.wrapping_add(1);
            self.registers.set_bc(new_value);
          },
          IncDecTarget::DE => {
            let value = self.registers.get_de();
            let new_value = value.wrapping_add(1);
            self.registers.set_de(new_value);
          },
          IncDecTarget::HL => {
            let value = self.registers.get_hl();
            let new_value = value.wrapping_add(1);
            self.registers.set_hl(new_value);
          },
          _ => { panic!("TODO: support more targets") }
        }
        self.pc.wrapping_add(1)
      },
      Instruction::DEC(target) => {
        match target {
          IncDecTarget::B => {
            let value  = self.registers.b;
            let new_value = self.dec_8bit(value);
            self.registers.b = new_value;
          },
          IncDecTarget::C => {
            let value  = self.registers.c;
            let new_value = self.dec_8bit(value);
            self.registers.c = new_value;
          },
          IncDecTarget::D => {
            let value  = self.registers.d;
            let new_value = self.dec_8bit(value);
            self.registers.d = new_value;
          },
          IncDecTarget::E => {
            let value  = self.registers.e;
            let new_value = self.dec_8bit(value);
            self.registers.e = new_value;
          },
          IncDecTarget::H => {
            let value  = self.registers.h;
            let new_value = self.dec_8bit(value);
            self.registers.h = new_value;
          },
          IncDecTarget::BC => {
            let new_value = self.dec_16bit(self.registers.get_bc());
            self.registers.set_bc(new_value);
          },
          IncDecTarget::DE => {
            let new_value = self.dec_16bit(self.registers.get_de());
            self.registers.set_de(new_value);
          },
          IncDecTarget::HL => {
            let new_value = self.dec_16bit(self.registers.get_hl());
            self.registers.set_hl(new_value);
          },
          _ => { panic!("TODO: support more targets") }
        }
        self.pc.wrapping_add(1)
      },
      Instruction::RLCA => {
        let value = self.registers.a;
        let seventh_bit = value >> 7;
        let new_value = (value << 1) | seventh_bit;
        self.registers.a = new_value;
        self.set_rotation_flags(seventh_bit);
        self.pc.wrapping_add(1)
      },
      Instruction::RRCA => {
        let value = self.registers.a;
        let zeroth_bit = value & 1;
        let new_value = (zeroth_bit << 7) | (value >> 1);
        self.registers.a = new_value;
        self.set_rotation_flags(zeroth_bit);
        self.pc.wrapping_add(1)
      },
      Instruction::RLA => {
        let value = self.registers.a;
        let seventh_bit = value >> 7;
        let new_value = (value << 1) | self.registers.f.carry as u8;
        self.registers.a = new_value;
        self.set_rotation_flags(seventh_bit);
        self.pc.wrapping_add(1)
      },
      Instruction::RRA => {
        let value = self.registers.a;
        let zero_bit = value & 1;
        let carry_flag = self.registers.f.carry as u8;
        let new_value = (value >> 1) | carry_flag << 7;
        self.registers.a = new_value;
        self.set_rotation_flags(zero_bit);
        self.pc.wrapping_add(1)
      },
      Instruction::DAA => {
        let mut offset = 0;
        let mut should_carry = false;

        let a_value = self.registers.a;
        let half_carry = self.registers.f.half_carry;
        let carry = self.registers.f.carry;
        let subtract = self.registers.f.subtract;
    
        if (!subtract && (a_value & 0x0F) > 0x09) || half_carry {
          offset |= 0x06;
        }
    
        if (!subtract && a_value > 0x99) || carry {
          offset |= 0x60;
          should_carry = true;
        }
    
        let output = if !subtract {
            a_value.wrapping_add(offset)
        } else {
            a_value.wrapping_sub(offset)
        };

        self.registers.a = output;

        self.registers.set_f(
          Some(output == 0),
          None,
          Some(false),
          Some(should_carry)
        );

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
    self.registers.set_f(
        None,
        Some(false),
        Some((value & 0x0FFF) + (hl_value & 0x0FFF) > 0x0FFF),
        Some(did_overflow)
    );
    self.registers.set_hl(new_value);
  }

  fn inc_8bit(&mut self, value: u8) -> u8 {
    let new_value = value.wrapping_add(1);
    self.registers.set_f(
        Some(new_value == 0),
        Some(false),
        Some((value & 0x0F) + 1 > 0x0F),
        None
    );
    new_value
  }

  fn dec_8bit(&mut self, value: u8) -> u8 {
    let new_value = value.wrapping_sub(1);
    self.registers.set_f(
        Some(new_value == 0),
        Some(true),
        Some((value & 0x0F) == 0),
        None
    );
    new_value
  }

  fn dec_16bit(&mut self, value: u16) -> u16 {
    let new_value = value.wrapping_sub(1);
    new_value
  }

  fn read_immediate_16bit(&mut self) -> u16 {
    self.bus.read_byte(self.pc + 1) as u16 | (self.bus.read_byte(self.pc + 2) as u16) << 8
  }

  fn set_rotation_flags(&mut self, carry: u8) {
    self.registers.set_f(
        Some(false),
        Some(false),
        Some(false),
        Some(carry != 0)
    );
  }
}

