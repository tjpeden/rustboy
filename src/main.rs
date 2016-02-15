use std::env;
use std::fs;
use std::io::Read;

fn main() {
  let game_name = env::args().nth(1).unwrap();
  let mut game = fs::File::open(&game_name).unwrap();
  let mut game_buffer = Vec::new();

  game.read_to_end(&mut game_buffer).unwrap();

  let game_buffer = game_buffer;
}
