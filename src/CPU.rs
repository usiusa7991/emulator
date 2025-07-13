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
      Instruction::ADD(add_type) => {
        match add_type {
          AddType::Byte(target, source) => {
            let source_value = match source {
              AddByteSource::A => self.registers.a,
              AddByteSource::B => self.registers.b,
              AddByteSource::C => self.registers.c,
              AddByteSource::D => self.registers.d,
              AddByteSource::E => self.registers.e,
              AddByteSource::H => self.registers.h,
              AddByteSource::L => self.registers.l,
              AddByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
              _ => panic!("TODO: implement other AddByteSource"),
            };
            match target {
              AddByteTarget::A => self.add_to_a(source_value),
              _ => panic!("TODO: implement other AddByteTarget"),
            };
          },
          AddType::TwoByte(target, source) => {
            let source_value = match source {
              AddTwoByteSource::BC => self.registers.get_bc(),
              AddTwoByteSource::DE => self.registers.get_de(),
              AddTwoByteSource::HL => self.registers.get_hl(),
              AddTwoByteSource::SP => self.sp,
              _ => panic!("TODO: implement other AddTwoByteSource"),
            };
            match target {
              AddTwoByteTarget::HL => self.add_to_hl(source_value),
              _ => panic!("TODO: implement other AddTwoByteTarget"),
            };
          }
        }
        self.pc.wrapping_add(1)
      },
      Instruction::ADC(target, source) => {
        let source_value = match source {
          AdcSource::A => self.registers.a,
          AdcSource::B => self.registers.b,
          AdcSource::C => self.registers.c,
          AdcSource::D => self.registers.d,
          AdcSource::E => self.registers.e,
          AdcSource::H => self.registers.h,
          AdcSource::L => self.registers.l,
          AdcSource::D8 => self.read_next_byte(),
          AdcSource::HLI => self.bus.read_byte(self.registers.get_hl()),
          _ => panic!("TODO: implement other AdcSource"),
        };
        match target {
          AdcTarget::A => {
            let carry = if self.registers.f.carry { 1 } else { 0 };
            self.add_to_a(source_value.wrapping_add(carry));
          },
          _ => panic!("TODO: implement other AdcTarget"),
        };
        match source {
          AdcSource::D8 => self.pc.wrapping_add(2),
          _ => self.pc.wrapping_add(1),
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
      Instruction::SUB(source) => {
        let source_value = match source {
          SubSource::A => self.registers.a,
          SubSource::B => self.registers.b,
          SubSource::C => self.registers.c,
          SubSource::D => self.registers.d,
          SubSource::E => self.registers.e,
          SubSource::H => self.registers.h,
          SubSource::L => self.registers.l,
          SubSource::HLI => self.bus.read_byte(self.registers.get_hl()),
          _ => panic!("TODO: implement other SubSource"),
        };
        self.sub_a(source_value);
        match source {
          _ => self.pc.wrapping_add(1),
        }
      },
      Instruction::SBC(source) => {
        let source_value = match source {
          SbcSource::A => self.registers.a,
          SbcSource::B => self.registers.b,
          SbcSource::C => self.registers.c,
          SbcSource::D => self.registers.d,
          SbcSource::E => self.registers.e,
          SbcSource::H => self.registers.h,
          SbcSource::L => self.registers.l,
          SbcSource::HLI => self.bus.read_byte(self.registers.get_hl()),
          _ => panic!("TODO: implement other SbcSource"),
        };
        self.sbc_a(source_value);
        match source {
          _ => self.pc.wrapping_add(1),
        }
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
          JumpRelativeConditions::NoCarryFlag => {
            self.registers.f.carry == false
          },
          JumpRelativeConditions::CarryFlag => {
            self.registers.f.carry == true
          },
        };

        if condition_flag {
          (((self.pc.wrapping_add(2)) as i32).wrapping_add(skip_counts as i32)) as u16
        } else {
          self.pc.wrapping_add(2)
        }
      },
      Instruction::RET(conditions) => {
        let condition_flag = match conditions {
          RetConditions::NoZeroFlag => !self.registers.f.zero,
          RetConditions::ZeroFlag => self.registers.f.zero,
          RetConditions::NoCarryFlag => !self.registers.f.carry,
          RetConditions::CarryFlag => self.registers.f.carry,
          RetConditions::Always => true
        };
        if condition_flag {
          self.pop()
        } else {
          self.pc.wrapping_add(1)
        }
      },
      Instruction::LD(load_type) => {
        match load_type {
          LoadType::Byte(target, source) => {
            let source_value = match source {
              LoadByteSource::A => self.registers.a,
              LoadByteSource::B => self.registers.b,
              LoadByteSource::C => self.registers.c,
              LoadByteSource::D => self.registers.d,
              LoadByteSource::E => self.registers.e,
              LoadByteSource::H => self.registers.h,
              LoadByteSource::L => self.registers.l,
              LoadByteSource::D8 => self.read_next_byte(),
              LoadByteSource::BCI => self.bus.read_byte(self.registers.get_bc()),
              LoadByteSource::DEI => self.bus.read_byte(self.registers.get_de()),
              LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
              LoadByteSource::HLIP => {
                let hl_value = self.registers.get_hl();
                self.registers.set_hl(hl_value.wrapping_add(1));
                self.bus.read_byte(hl_value)
              },
              LoadByteSource::HLIM => {
                let hl_value = self.registers.get_hl();
                self.registers.set_hl(hl_value.wrapping_sub(1));
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
              LoadByteTarget::L => self.registers.l = source_value,
              LoadByteTarget::BCI => self.bus.write_byte(self.registers.get_bc(), source_value),
              LoadByteTarget::DEI => self.bus.write_byte(self.registers.get_de(), source_value),
              LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
              LoadByteTarget::HLIP => {
                let hl_value = self.registers.get_hl();
                self.bus.write_byte(self.registers.get_hl(), source_value);
                self.registers.set_hl(hl_value.wrapping_add(1))
              },
              LoadByteTarget::HLIM => {
                let hl_value = self.registers.get_hl();
                self.bus.write_byte(self.registers.get_hl(), source_value);
                self.registers.set_hl(hl_value.wrapping_sub(1))
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
              LoadTwoByteTarget::SP => {
                let value = self.read_immediate_16bit();
                self.sp = value;
              },
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
      Instruction::INC(target) => {
        match target {
          IncDecTarget::A => {
            let new_value = self.inc_8bit(self.registers.a);
            self.registers.a = new_value;
          },
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
          IncDecTarget::L => {
            let new_value = self.inc_8bit(self.registers.l);
            self.registers.l = new_value;
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
          IncDecTarget::HLI => {
            let address = self.registers.get_hl();
            let value = self.bus.read_byte(address);
            let new_value = self.inc_8bit(value);
            self.bus.write_byte(address, new_value);
          },
          IncDecTarget::SP => {
            let value = self.sp;
            self.sp = value.wrapping_add(1);
          },
          _ => { panic!("TODO: support more targets") }
        }
        self.pc.wrapping_add(1)
      },
      Instruction::DEC(target) => {
        match target {
          IncDecTarget::A => {
            let value = self.registers.a;
            let new_value = self.dec_8bit(value);
            self.registers.a = new_value;
          },
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
          IncDecTarget::L => {
            let value  = self.registers.l;
            let new_value = self.dec_8bit(value);
            self.registers.l = new_value;
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
          IncDecTarget::HLI => {
            let address = self.registers.get_hl();
            let value = self.bus.read_byte(address);
            let new_value = self.dec_8bit(value);
            self.bus.write_byte(address, new_value);
          },
          IncDecTarget::SP => {
            let new_value = self.dec_16bit(self.sp);
            self.sp = new_value;
          },
          _ => { panic!("TODO: support more targets") }
        }
        self.pc.wrapping_add(1)
      },
      Instruction::AND(source) => {
        let source_value = match source {
          AndSource::A => self.registers.a,
          AndSource::B => self.registers.b,
          AndSource::C => self.registers.c,
          AndSource::D => self.registers.d,
          AndSource::E => self.registers.e,
          AndSource::H => self.registers.h,
          AndSource::L => self.registers.l,
          AndSource::HLI => self.bus.read_byte(self.registers.get_hl()),
        };
        self.and_a(source_value);
        match source {
          _ => self.pc.wrapping_add(1),
        }
      },
      Instruction::XOR(source) => {
        let source_value = match source {
          XorSource::A => self.registers.a,
          XorSource::B => self.registers.b,
          XorSource::C => self.registers.c,
          XorSource::D => self.registers.d,
          XorSource::E => self.registers.e,
          XorSource::H => self.registers.h,
          XorSource::L => self.registers.l,
          XorSource::HLI => self.bus.read_byte(self.registers.get_hl()),
        };
        self.xor_a(source_value);
        match source {
          _ => self.pc.wrapping_add(1),
        }
      },
      Instruction::OR(source) => {
        let source_value = match source {
          OrSource::A => self.registers.a,
          OrSource::B => self.registers.b,
          OrSource::C => self.registers.c,
          OrSource::D => self.registers.d,
          OrSource::E => self.registers.e,
          OrSource::H => self.registers.h,
          OrSource::L => self.registers.l,
          OrSource::HLI => self.bus.read_byte(self.registers.get_hl()),
        };
        self.or_a(source_value);
        match source {
          _ => self.pc.wrapping_add(1),
        }
      },
      Instruction::CP(source) => {
        let source_value = match source {
          CpSource::A => self.registers.a,
          CpSource::B => self.registers.b,
          CpSource::C => self.registers.c,
          CpSource::D => self.registers.d,
          CpSource::E => self.registers.e,
          CpSource::H => self.registers.h,
          CpSource::L => self.registers.l,
          CpSource::HLI => self.bus.read_byte(self.registers.get_hl()),
        };
        self.cp_a(source_value);
        match source {
          _ => self.pc.wrapping_add(1),
        }
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
      Instruction::CPL => {
        self.registers.a = !self.registers.a;
        self.registers.set_f(
          None,
          Some(true),
          Some(true),
          None
        );
        self.pc.wrapping_add(1)
      },
      Instruction::SCF => {
        self.registers.set_f(
          None,
          Some(false),
          Some(false),
          Some(true)
        );
        self.pc.wrapping_add(1)
      },
      Instruction::CCF => {
        self.registers.set_f(
          None,
          Some(false),
          Some(false),
          Some(!self.registers.f.carry)
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

  fn add_to_a(&mut self, value: u8) {
    let a_value= self.registers.a;
    let (result, did_overflow) = a_value.overflowing_add(value);
    self.registers.set_f(
        Some(result == 0),
        Some(false),
        Some((a_value & 0x0F) + (value & 0x0F) > 0x0F),
        Some(did_overflow)
    );
    self.registers.a = result;
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

  fn adc_to_a(&mut self, value: u8) {
    let a_value = self.registers.a;
    let carry = if self.registers.f.carry { 1 } else { 0 };
    let (result, did_overflow) = a_value.overflowing_add(value.wrapping_add(carry));
    self.registers.set_f(
        Some(result == 0),
        Some(false),
        Some((a_value & 0x0F) + (value & 0x0F) + carry > 0x0F),
        Some(did_overflow)
    );
    self.registers.a = result;
  }

  fn sub_a(&mut self, value: u8) {
    let a = self.registers.a;
    let (result, borrow) = a.overflowing_sub(value);
    self.registers.set_f(
      Some(result == 0),
      Some(true),
      Some((a & 0x0F) < (value & 0x0F)),
      Some(borrow)
    );
    self.registers.a = result;
  }

  fn sbc_a(&mut self, value: u8) {
    let a = self.registers.a;
    let carry = if self.registers.f.carry { 1 } else { 0 };
    let (result, borrow) = a.overflowing_sub(value.wrapping_add(carry));
    self.registers.set_f(
      Some(result == 0),
      Some(true),
      Some((a & 0x0F) < (value & 0x0F) + carry),
      Some(borrow)
    );
    self.registers.a = result;
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

  fn and_a(&mut self, value: u8) {
    let result = self.registers.a & value;
    self.registers.a = result;
    self.registers.set_f(
        Some(result == 0),
        Some(false),
        Some(true),
        Some(false)
    );
  }

  fn xor_a(&mut self, value: u8) {
    let result = self.registers.a ^ value;
    self.registers.a = result;
    self.registers.set_f(
        Some(result == 0),
        Some(false),
        Some(false),
        Some(false)
    );
  }

  fn or_a(&mut self, value: u8) {
    let result = self.registers.a | value;
    self.registers.a = result;
    self.registers.set_f(
        Some(result == 0),
        Some(false),
        Some(false),
        Some(false)
    );
  }

  fn cp_a(&mut self, value: u8) {
    let result = self.registers.a.wrapping_sub(value);
    self.registers.set_f(
        Some(result == 0),
        Some(true),
        Some((self.registers.a & 0x0F) < (value & 0x0F)),
        Some(self.registers.a < value)
    );
  }
}

