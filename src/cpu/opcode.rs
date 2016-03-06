enum_from_primitive! {
  #[derive(Debug)]
  pub enum Opcode {
    // 8-bit load
    LoadHLDecA = 0x32,

    // 16-bit load
    LoadHL = 0x21,
    LoadSP = 0x31,

    // 8-bit math
    XorA = 0xAF,

    Special = 0xCB,
  }
}

enum_from_primitive! {
  #[derive(Debug)]
  pub enum SpecialOpcode {
    Bit7H = 0x7C,
  }
}
