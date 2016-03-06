use super::cpu;
use super::memory::MemoryMap;

pub struct GameBoy {
  cpu: cpu::Cpu<MemoryMap>,
}

impl GameBoy {
  pub fn new(bootrom: Box<[u8]>) -> GameBoy {
    let memory_map = MemoryMap::new(bootrom);

    GameBoy {
      cpu: cpu::Cpu::new(memory_map),
    }
  }

  pub fn run(&mut self) {
    loop {
      self.cpu.step();
    }
  }
}