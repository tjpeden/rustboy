enum_from_primitive! {
  #[derive(Debug)]
  pub enum Opcode {
    // Jumps/Calls/Returns
    JumpNZ = 0x20,
    CallAddrImm = 0xCD,
    Return = 0xC9,

    // 8-bit load
    LoadAIntoC = 0x4f,
    LoadAddrDeIntoA = 0x1A,
    LoadImmIntoA = 0x3E,
    LoadImmIntoB = 0x06,
    LoadImmIntoC = 0x0E,
    LoadAintoAddrHlAndInc = 0x22,
    LoadAIntoAddrHlAndDec = 0x32,
    LoadAIntoAddrC = 0xE2,
    LoadAIntoAddrHl = 0x77,
    LoadAIntoAddrImm = 0xE0,

    // 16-bit load
    LoadImmIntoDe = 0x11,
    LoadImmIntoHl = 0x21,
    LoadImmIntoSp = 0x31,
    PushBc = 0xC5,
    PopBc = 0xC1,

    // 8-bit math
    DecrementB = 0x05,
    IncrementC = 0x0C,
    XorA = 0xAF,

    // 16-bit math
    IncrementHl = 0x23,

    Special = 0xCB,
    RotateLeftA = 0x17,
  }
}

enum_from_primitive! {
  #[derive(Debug)]
  pub enum SpecialOpcode {
    RotateLeftC = 0x11,
    Bit7H = 0x7C,
  }
}
