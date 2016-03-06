enum_from_primitive! {
  #[derive(Debug)]
  pub enum Opcode {
    // 16-bit load
    LoadHL = 0x21,
    LoadSP = 0x31,

    // 8-bit math
    XorA = 0xAF,
  }
}
