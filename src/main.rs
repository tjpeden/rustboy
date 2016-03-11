extern crate byteorder;
extern crate num;
#[macro_use]
extern crate enum_primitive;

mod cpu;
mod game_boy;
mod memory;
mod io_ports;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {
  let bootrom_name = env::args().nth(1).unwrap();
  let gamerom_name = env::args().nth(2).unwrap();

  let bootrom = read_binary(bootrom_name);
  let gamerom = read_binary(gamerom_name);

  let mut game_boy = game_boy::GameBoy::new(bootrom);

  game_boy.run();
}

fn read_binary<P: AsRef<Path>>(path: P) -> Box<[u8]> {
  let mut file = fs::File::open(path).unwrap();
  let mut file_buf = Vec::new();

  file.read_to_end(&mut file_buf).unwrap();

  file_buf.into_boxed_slice()
}
