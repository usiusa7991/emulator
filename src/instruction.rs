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

pub enum RetConditions {
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
  ADD(AddType),
  ADC(AdcTarget, AdcSource),
  SUB(SubSource),
  SBC(SbcSource),
  PUSH(StackTarget),
  POP(StackTarget),
  CALL(JumpTest),
  RLC(PrefixTarget),
  INC(IncDecTarget),
  DEC(IncDecTarget),
  AND(AndSource),
  XOR(XorSource),
  OR(OrSource),
  CP(CpSource),
  JP(JumpTest),
  JR(JumpRelativeConditions),
  RET(RetConditions),
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

pub enum AddByteTarget {
  A
}

pub enum AddByteSource {
  A, B, C, D, E, H, L, HLI
}

pub enum AddTwoByteTarget {
  HL
}

pub enum AddTwoByteSource {
  BC, DE, HL, SP
}

pub enum AddType {
  Byte(AddByteTarget, AddByteSource),
  TwoByte(AddTwoByteTarget, AddTwoByteSource)
}

pub enum AdcTarget {
  A
}

pub enum AdcSource {
  A, B, C, D, E, H, L, D8, HLI
}

pub enum SubSource {
  A, B, C, D, E, H, L, D8, HLI
}

pub enum SbcSource {
  A, B, C, D, E, H, L, D8, HLI
}

pub enum IncDecTarget {
  A, B, C, D, E, H, L, BC, DE, HL, HLI, SP
}

pub enum AndSource {
  A, B, C, D, E, H, L, HLI
}

pub enum XorSource {
  A, B, C, D, E, H, L, HLI
}

pub enum OrSource {
  A, B, C, D, E, H, L, HLI
}

pub enum CpSource {
  A, B, C, D, E, H, L, HLI
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
      0x09 => Some(Instruction::ADD(AddType::TwoByte(AddTwoByteTarget::HL, AddTwoByteSource::BC))),
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
      0x19 => Some(Instruction::ADD(AddType::TwoByte(AddTwoByteTarget::HL, AddTwoByteSource::DE))),
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
      0x29 => Some(Instruction::ADD(AddType::TwoByte(AddTwoByteTarget::HL, AddTwoByteSource::HL))),
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
      0x39 => Some(Instruction::ADD(AddType::TwoByte(AddTwoByteTarget::HL, AddTwoByteSource::SP))),
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
      0x80 => Some(Instruction::ADD(AddType::Byte(AddByteTarget::A, AddByteSource::B))),
      0x81 => Some(Instruction::ADD(AddType::Byte(AddByteTarget::A, AddByteSource::C))),
      0x82 => Some(Instruction::ADD(AddType::Byte(AddByteTarget::A, AddByteSource::D))),
      0x83 => Some(Instruction::ADD(AddType::Byte(AddByteTarget::A, AddByteSource::E))),
      0x84 => Some(Instruction::ADD(AddType::Byte(AddByteTarget::A, AddByteSource::H))),
      0x85 => Some(Instruction::ADD(AddType::Byte(AddByteTarget::A, AddByteSource::L))),
      0x86 => Some(Instruction::ADD(AddType::Byte(AddByteTarget::A, AddByteSource::HLI))),
      0x87 => Some(Instruction::ADD(AddType::Byte(AddByteTarget::A, AddByteSource::A))),
      0x88 => Some(Instruction::ADC(AdcTarget::A, AdcSource::B)),
      0x89 => Some(Instruction::ADC(AdcTarget::A, AdcSource::C)),
      0x8A => Some(Instruction::ADC(AdcTarget::A, AdcSource::D)),
      0x8B => Some(Instruction::ADC(AdcTarget::A, AdcSource::E)),
      0x8C => Some(Instruction::ADC(AdcTarget::A, AdcSource::H)),
      0x8D => Some(Instruction::ADC(AdcTarget::A, AdcSource::L)),
      0x8E => Some(Instruction::ADC(AdcTarget::A, AdcSource::HLI)),
      0x8F => Some(Instruction::ADC(AdcTarget::A, AdcSource::A)),
      0x90 => Some(Instruction::SUB(SubSource::B)),
      0x91 => Some(Instruction::SUB(SubSource::C)),
      0x92 => Some(Instruction::SUB(SubSource::D)),
      0x93 => Some(Instruction::SUB(SubSource::E)),
      0x94 => Some(Instruction::SUB(SubSource::H)),
      0x95 => Some(Instruction::SUB(SubSource::L)),
      0x96 => Some(Instruction::SUB(SubSource::HLI)),
      0x97 => Some(Instruction::SUB(SubSource::A)),
      0x98 => Some(Instruction::SBC(SbcSource::B)),
      0x99 => Some(Instruction::SBC(SbcSource::C)),
      0x9A => Some(Instruction::SBC(SbcSource::D)),
      0x9B => Some(Instruction::SBC(SbcSource::E)),
      0x9C => Some(Instruction::SBC(SbcSource::H)),
      0x9D => Some(Instruction::SBC(SbcSource::L)),
      0x9E => Some(Instruction::SBC(SbcSource::HLI)),
      0x9F => Some(Instruction::SBC(SbcSource::A)),
      0xA0 => Some(Instruction::AND(AndSource::B)),
      0xA1 => Some(Instruction::AND(AndSource::C)),
      0xA2 => Some(Instruction::AND(AndSource::D)),
      0xA3 => Some(Instruction::AND(AndSource::E)),
      0xA4 => Some(Instruction::AND(AndSource::H)),
      0xA5 => Some(Instruction::AND(AndSource::L)),
      0xA6 => Some(Instruction::AND(AndSource::HLI)),
      0xA7 => Some(Instruction::AND(AndSource::A)),
      0xA8 => Some(Instruction::XOR(XorSource::B)),
      0xA9 => Some(Instruction::XOR(XorSource::C)),
      0xAA => Some(Instruction::XOR(XorSource::D)),
      0xAB => Some(Instruction::XOR(XorSource::E)),
      0xAC => Some(Instruction::XOR(XorSource::H)),
      0xAD => Some(Instruction::XOR(XorSource::L)),
      0xAE => Some(Instruction::XOR(XorSource::HLI)),
      0xAF => Some(Instruction::XOR(XorSource::A)),
      0xB0 => Some(Instruction::OR(OrSource::B)),
      0xB1 => Some(Instruction::OR(OrSource::C)),
      0xB2 => Some(Instruction::OR(OrSource::D)),
      0xB3 => Some(Instruction::OR(OrSource::E)),
      0xB4 => Some(Instruction::OR(OrSource::H)),
      0xB5 => Some(Instruction::OR(OrSource::L)),
      0xB6 => Some(Instruction::OR(OrSource::HLI)),
      0xB7 => Some(Instruction::OR(OrSource::A)),
      0xB8 => Some(Instruction::CP(CpSource::B)),
      0xB9 => Some(Instruction::CP(CpSource::C)),
      0xBA => Some(Instruction::CP(CpSource::D)),
      0xBB => Some(Instruction::CP(CpSource::E)),
      0xBC => Some(Instruction::CP(CpSource::H)),
      0xBD => Some(Instruction::CP(CpSource::L)),
      0xBE => Some(Instruction::CP(CpSource::HLI)),
      0xBF => Some(Instruction::CP(CpSource::A)),
      0xC0 => Some(Instruction::RET(RetConditions::NoZeroFlag)),
      0xC1 => Some(Instruction::POP(StackTarget::BC)),
      0xC8 => Some(Instruction::RET(RetConditions::ZeroFlag)),
      0xC9 => Some(Instruction::RET(RetConditions::Always)),
      0xD0 => Some(Instruction::RET(RetConditions::NoCarryFlag)),
      0xD1 => Some(Instruction::POP(StackTarget::DE)),
      0xD8 => Some(Instruction::RET(RetConditions::CarryFlag)),
      0xE1 => Some(Instruction::POP(StackTarget::HL)),
      0xF1 => Some(Instruction::POP(StackTarget::AF)),
      _ => None
    }
  }
}
