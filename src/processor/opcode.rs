enum_from_primitive! {
  #[derive(Debug)]
  pub enum Opcode {
    // Control
    NoOp = 0x00,
    // Stop = 0x10,
    // Halt = 0x76,
    Special = 0xCB,

    // Flow Control
    Jump = 0xC3,
    JumpNonZero = 0xC2,
    JumpZero = 0xCA,
    JumpNonCarry = 0xD2,
    JumpCarry = 0xDA,
    JumpRelative = 0x18,
    JumpRelativeNonZero = 0x20,
    JumpRelativeZero = 0x28,
    JumpRelativeNonCarry = 0x30,
    JumpRelativeCarry = 0x38,
    Return = 0xC9,
    CallAddrImm = 0xCD,

    // 8-bit Load
    LoadAIntoC = 0x4f,
    LoadAIntoD = 0x57,
    LoadAIntoH = 0x67,
    LoadEIntoA = 0x7B,
    LoadImmIntoB = 0x06,
    LoadImmIntoC = 0x0E,
    LoadImmIntoE = 0x1E,
    LoadImmIntoL = 0x2E,
    LoadImmIntoA = 0x3E,
    LoadAIntoAddrC = 0xE2,
    LoadAIntoAddrBc = 0x02,
    LoadAIntoAddrHl = 0x77,
    LoadAddrBcIntoA = 0x0A,
    LoadAddrDeIntoA = 0x1A,
    LoadAIntoAddrImm = 0xEA,
    LoadAIntoAddrHlAndInc = 0x22,
    LoadAIntoAddrHlAndDec = 0x32,
    LoadAIntoAddrImmIO = 0xE0,
    LoadAddrImmIOIntoA = 0xF0,

    // 16-bit Load
    LoadImmIntoDe = 0x11,
    LoadImmIntoHl = 0x21,
    LoadImmIntoSp = 0x31,
    PopBc = 0xC1,
    PushBc = 0xC5,

    // 8-bit Math
    IncrementB = 0x04,
    IncrementC = 0x0C,
    DecrementB = 0x05,
    DecrementC = 0x0D,
    DecrementA = 0x3D,
    SubtractL = 0x95,
    XorA = 0xAF,
    CompareImm = 0xFE,

    // 16-bit Math
    IncrementDe = 0x13,
    IncrementHl = 0x23,

    // 8-bit Shift/Rotate
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
