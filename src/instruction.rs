pub enum JumpTest {
  NotZero,
  Zero,
  NotCarry,
  Carry,
  Always
}

pub enum JumpRelativeConditions {
  NoZeroFlag,
  ZeroFlag,
  NoCarryFlag,
  CarryFlag,
  Always
}

pub enum LoadByteTarget {
  A, B, C, D, E, H, L, BCI, DEI, HLI, HLIP, HLIM
}

pub enum LoadByteSource {
  A, B, C, D, E, H, L, D8, BCI, DEI, HLI, HLIP, HLIM
}

pub enum LoadTwoByteTarget {
  BC, DE, HL, SP, A16
}

pub enum LoadTwoByteSource {
  D16, SP
}

pub enum LoadType {
  Byte(LoadByteTarget, LoadByteSource),
  TwoByte(LoadTwoByteTarget, LoadTwoByteSource)
}

pub enum Instruction {
  NOP,
  ADD(AddTarget, AddSource),
  PUSH(StackTarget),
  POP(StackTarget),
  CALL(JumpTest),
  RET(JumpTest),
  RLC(PrefixTarget),
  INC(IncDecTarget),
  DEC(IncDecTarget),
  JP(JumpTest),
  JR(JumpRelativeConditions),
  LD(LoadType),
  RLCA,
  RRCA,
  RLA,
  RRA,
  DAA,
  CPL,
  SCF,
  CCF
}

pub enum AddTarget {
	HL
}

pub enum AddSource {
	BC, DE, HL, SP
}

pub enum IncDecTarget {
	A, B, C, D, E, H, L, BC, DE, HL, HLI, SP
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
      0x01 => Some(Instruction::LD(LoadType::TwoByte(LoadTwoByteTarget::BC, LoadTwoByteSource::D16))),
      0x02 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::BCI, LoadByteSource::A))),
      0x03 => Some(Instruction::INC(IncDecTarget::BC)),
      0x04 => Some(Instruction::INC(IncDecTarget::B)),
      0x05 => Some(Instruction::DEC(IncDecTarget::B)),
      0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
      0x07 => Some(Instruction::RLCA),
      0x08 => Some(Instruction::LD(LoadType::TwoByte(LoadTwoByteTarget::A16, LoadTwoByteSource::SP))),
      0x09 => Some(Instruction::ADD(AddTarget::HL, AddSource::BC)),
      0x0A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::BCI))),
      0x0B => Some(Instruction::DEC(IncDecTarget::BC)),
      0x0C => Some(Instruction::INC(IncDecTarget::C)),
      0x0D => Some(Instruction::DEC(IncDecTarget::C)),
      0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
      0x0F => Some(Instruction::RRCA),
      0x11 => Some(Instruction::LD(LoadType::TwoByte(LoadTwoByteTarget::DE, LoadTwoByteSource::D16))),
      0x12 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::DEI, LoadByteSource::A))),
      0x13 => Some(Instruction::INC(IncDecTarget::DE)),
      0x14 => Some(Instruction::INC(IncDecTarget::D)),
      0x15 => Some(Instruction::DEC(IncDecTarget::D)),
      0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
      0x17 => Some(Instruction::RLA),
      0x18 => Some(Instruction::JR(JumpRelativeConditions::Always)),
      0x19 => Some(Instruction::ADD(AddTarget::HL, AddSource::DE)),
      0x1A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::DEI))),
      0x1B => Some(Instruction::DEC(IncDecTarget::DE)),
      0x1C => Some(Instruction::INC(IncDecTarget::E)),
      0x1D => Some(Instruction::DEC(IncDecTarget::E)),
      0x1E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),
      0x1F => Some(Instruction::RRA),
      0x20 => Some(Instruction::JR(JumpRelativeConditions::NoZeroFlag)),
      0x21 => Some(Instruction::LD(LoadType::TwoByte(LoadTwoByteTarget::HL, LoadTwoByteSource::D16))),
      0x22 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLIP, LoadByteSource::A))),
      0x23 => Some(Instruction::INC(IncDecTarget::HL)),
      0x24 => Some(Instruction::INC(IncDecTarget::H)),
      0x25 => Some(Instruction::DEC(IncDecTarget::H)),
      0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),
      0x27 => Some(Instruction::DAA),
      0x28 => Some(Instruction::JR(JumpRelativeConditions::ZeroFlag)),
      0x29 => Some(Instruction::ADD(AddTarget::HL, AddSource::HL)),
      0x2A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLIP))),
      0x2B => Some(Instruction::DEC(IncDecTarget::HL)),
      0x2C => Some(Instruction::INC(IncDecTarget::L)),
      0x2D => Some(Instruction::DEC(IncDecTarget::L)),
      0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
      0x2F => Some(Instruction::CPL),
      0x30 => Some(Instruction::JR(JumpRelativeConditions::NoCarryFlag)),
      0x31 => Some(Instruction::LD(LoadType::TwoByte(LoadTwoByteTarget::SP, LoadTwoByteSource::D16))),
      0x32 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLIM, LoadByteSource::A))),
      0x33 => Some(Instruction::INC(IncDecTarget::SP)),
      0x34 => Some(Instruction::INC(IncDecTarget::HLI)),
      0x35 => Some(Instruction::DEC(IncDecTarget::HLI)),
      0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D8))),
      0x37 => Some(Instruction::SCF),
      0x38 => Some(Instruction::JR(JumpRelativeConditions::CarryFlag)),
      0x39 => Some(Instruction::ADD(AddTarget::HL, AddSource::SP)),
      0x3A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLIM))),
      0x3B => Some(Instruction::DEC(IncDecTarget::SP)),
      0x3C => Some(Instruction::INC(IncDecTarget::A)),
      0x3D => Some(Instruction::DEC(IncDecTarget::A)),
      0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),
      0x3F => Some(Instruction::CCF),
      0x40 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),
      0x41 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))),
      0x42 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),
      0x43 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))),
      0x44 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))),
      0x45 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))),
      0x46 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::HLI))),
      0x47 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))),
      0x48 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))),
      0x49 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))),
      0x4A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))),
      0x4B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))),
      0x4C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))),
      0x4D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))),
      0x4E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HLI))),
      0x4F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))),
      0x50 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))),
      0x51 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))),
      0x52 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))),
      0x53 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))),
      0x54 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))),
      0x55 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))),
      0x56 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HLI))),
      0x57 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))),
      0x58 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))),
      0x59 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))),
      0x5A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))),
      0x5B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))),
      0x5C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))),
      0x5D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))),
      0x5E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HLI))),
      0x5F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),
      0x60 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))),
      0x61 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))),
      0x62 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D))),
      0x63 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E))),
      0x64 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))),
      0x65 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))),
      0x66 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HLI))),
      0x67 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))),
      0x68 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))),
      0x69 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))),
      0x6A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))),
      0x6B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))),
      0x6C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))),
      0x6D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))),
      0x6E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HLI))),
      0x6F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))),
      0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::B))),
      0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::C))),
      0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D))),
      0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::E))),
      0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::H))),
      0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::L))),
      // TODO: 0x76 (HALT)
      0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A))),
      0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),
      0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),
      0x7A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),
      0x7B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),
      0x7C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),
      0x7D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),
      0x7E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI))),
      0x7F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),
      _ => None
    }
  }
}
