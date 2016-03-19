use super::processor;
// use super::display;
use super::memory::MemoryMap;

pub struct GameBoy {
  processor: processor::Processor<MemoryMap>,
  // display: display::Display<MemoryMap>,
}

impl GameBoy {
  pub fn new(bootrom: Box<[u8]>, gamerom: Box<[u8]>) -> GameBoy {
    let memory_map = MemoryMap::new(bootrom, gamerom);

    GameBoy {
      processor: processor::Processor::new(memory_map),
      // display: display::Display::new(memory_map),
    }
  }

  pub fn run(&mut self) {
    let mut tick = 0;
    loop {
      self.processor.step();
      print!("GameBoy: {}\n{:?}", tick, self.processor);
      tick += 1;
    }
  }
}
