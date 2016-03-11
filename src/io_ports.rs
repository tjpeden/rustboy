#[derive(Default)]
pub struct IOPorts;

impl IOPorts {
  pub fn read_nr_11(&self) -> u8 {
    unimplemented!()
  }

  pub fn write_nr_11(&mut self, value: u8) {
    println!("{:#08b} written to NR 11.", value);
    // panic!("Attempting to write {:#08b} to NR 11.", value);
  }

  pub fn read_nr_52(&self) -> u8 {
    0xF1
  }

  pub fn write_nr_52(&mut self, value: u8) {
    panic!("Attempting to write {:#08b} to NR 52.", value);
  }
}
