pub enum JumpTest {
  NotZero,
  Zero,
  NotCarry,
  Carry,
  Always
}

pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}

pub enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}

pub enum LoadType {
  Byte(LoadByteTarget, LoadByteSource),
}

pub enum Instruction {
  NOP,
  ADD(ArithmeticTarget),
  PUSH(StackTarget),
  POP(StackTarget),
  CALL(JumpTest),
  RET(JumpTest),
  RLC(PrefixTarget),
  INC(IncDecTarget),
  DEC(IncDecTarget),
  JP(JumpTest),
  LD(LoadType),
}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L, D8, HLI
}

pub enum IncDecTarget {
    A, B, C, D, E, H, L, HLI, BC, DE, HL, SP
}

pub enum PrefixTarget {
    A, B, C, D, E, H, L, HLI
}

pub enum StackTarget {
    AF, BC, DE, HL
}

impl Instruction {
  pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
    if prefixed {
      Instruction::from_byte_prefixed(byte)
    } else {
      Instruction::from_byte_not_prefixed(byte)
    }
  }

  fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      0x00 => Some(Instruction::RLC(PrefixTarget::B)),
      0x01 => Some(Instruction::RLC(PrefixTarget::C)),
      0x02 => Some(Instruction::RLC(PrefixTarget::D)),
      0x03 => Some(Instruction::RLC(PrefixTarget::E)),
      0x04 => Some(Instruction::RLC(PrefixTarget::H)),
      0x05 => Some(Instruction::RLC(PrefixTarget::L)),
      0x06 => Some(Instruction::RLC(PrefixTarget::HLI)),
      0x07 => Some(Instruction::RLC(PrefixTarget::A)),
      _ => None
    }
  }

  fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      0x00 => Some(Instruction::NOP),
      0x02 => Some(Instruction::INC(IncDecTarget::BC)),
      0x03 => Some(Instruction::INC(IncDecTarget::BC)),
      0x04 => Some(Instruction::INC(IncDecTarget::B)),
      0x05 => Some(Instruction::DEC(IncDecTarget::B)),
      0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
      _ => None
    }
  }
}