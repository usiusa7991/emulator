pub struct Registers {
  pub a: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub f: FlagsRegister,
  pub h: u8,
  pub l: u8,
}

#[derive(Clone, Copy)]
pub struct FlagsRegister {
  pub zero: bool,
  pub subtract: bool,
  pub half_carry: bool,
  pub carry: bool
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      a: 0,
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      f: FlagsRegister::from(0),
      h: 0,
      l: 0,
    }
  }

  pub fn get_af(&self) -> u16 {
    (self.a as u16) << 8 | u8::from(self.f.clone()) as u16
  }

  pub fn set_af(&mut self, value: u16) {
    self.a = ((value & 0xFF00) >> 8) as u8;
    self.f = FlagsRegister::from((value & 0xFF) as u8);
  }

  pub fn get_bc(&self) -> u16 {
    (self.b as u16) << 8
    | self.c as u16
  }

  pub fn set_bc(&mut self, value: u16) {
    self.b = ((value & 0xFF00) >> 8) as u8;
    self.c = (value & 0xFF) as u8;
  }

  pub fn get_de(&self) -> u16 {
    (self.d as u16) << 8
    | self.e as u16
  }

  pub fn set_de(&mut self, value: u16) {
    self.d = ((value & 0xFF00) >> 8) as u8;
    self.e = (value & 0xFF) as u8;
  }

  pub fn get_hl(&self) -> u16 {
    (self.h as u16) << 8
    | self.l as u16
  }

  pub fn set_hl(&mut self, value: u16) {
    self.h = ((value & 0xFF00) >> 8) as u8;
    self.l = (value & 0xFF) as u8;
  }

  pub fn set_f(
    &mut self,
    zero: Option<bool>,
    subtract: Option<bool>,
    half_carry: Option<bool>,
    carry: Option<bool>
  ) {
    if let Some(z) = zero {
      self.f.zero = z;
    }
    if let Some(n) = subtract {
      self.f.subtract = n;
    }
    if let Some(h) = half_carry {
      self.f.half_carry = h;
    }
    if let Some(c) = carry {
      self.f.carry = c;
    }
  }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8  {
  fn from(flag: FlagsRegister) -> u8 {
    (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
    (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
    (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
    (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
  }
}

impl std::convert::From<u8> for FlagsRegister {
  fn from(byte: u8) -> Self {
    let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
    let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
    let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
    let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

    FlagsRegister {
      zero,
      subtract,
      half_carry,
      carry
    }
  }
}
