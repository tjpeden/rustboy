enum_from_primitive! {
  #[derive(Debug)]
  pub enum Opcode {
    // Jumps/Calls/Returns
    JumpRelative = 0x18,
    JumpRelativeNZ = 0x20,
    JumpRelativeZ = 0x28,
    CallAddrImm = 0xCD,
    Return = 0xC9,

    // 8-bit load
    LoadAIntoC = 0x4f,
    LoadAIntoD = 0x57,
    LoadAIntoH = 0x67,
    LoadEIntoA = 0x7B,
    LoadAddrDeIntoA = 0x1A,
    LoadImmIntoA = 0x3E,
    LoadImmIntoB = 0x06,
    LoadImmIntoC = 0x0E,
    LoadImmIntoE = 0x1E,
    LoadImmIntoL = 0x2E,
    LoadAintoAddrHlAndInc = 0x22,
    LoadAIntoAddrHlAndDec = 0x32,
    LoadAIntoAddrHl = 0x77,
    LoadAIntoAddrC = 0xE2,
    LoadAIntoAddrImm = 0xEA,
    LoadAIntoAddrImmIO = 0xE0,
    LoadAddrImmIOIntoA = 0xF0,

    // 16-bit load
    LoadImmIntoDe = 0x11,
    LoadImmIntoHl = 0x21,
    LoadImmIntoSp = 0x31,
    PushBc = 0xC5,
    PopBc = 0xC1,

    // 8-bit math
    DecrementA = 0x3D,
    DecrementB = 0x05,
    DecrementC = 0x0D,
    IncrementB = 0x04,
    IncrementC = 0x0C,
    SubtractL = 0x95,
    XorA = 0xAF,
    CompareImm = 0xFE,

    // 16-bit math
    IncrementDe = 0x13,
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
