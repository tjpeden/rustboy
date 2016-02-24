enum_from_primitive! {
  #[derive(Debug)]
  pub enum Opcode {
    // 16-bit load
    LoadSPI16 = 0x31,

    // 8-bit math
    XorA = 0xAF,
  }
}
