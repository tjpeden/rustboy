enum_from_primitive! {
  #[derive(Debug)]
  pub enum Opcode {
    // Jumps
    JumpNZ = 0x20,

    // 8-bit load
    LoadAddrDeIntoA = 0x1A,
    LoadImmIntoC = 0x0E,
    LoadImmIntoA = 0x3E,
    LoadAIntoHlAndDec = 0x32,
    LoadAIntoAddrC = 0xE2,
    LoadAIntoAddrHl = 0x77,
    LoadAIntoAddrImm = 0xE0,

    // 16-bit load
    LoadImmIntoDe = 0x11,
    LoadImmIntoHl = 0x21,
    LoadImmIntoSp = 0x31,

    // 8-bit math
    IncrementC = 0x0C,
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
