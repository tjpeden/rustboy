use super::memory::{Memory};

#[allow(dead_code)]
pub struct Display<M: Memory> {
    memory: M,
}

#[allow(dead_code)]
impl<M: Memory> Display<M> {
    pub fn new(memory: M) -> Self {
        Display {
            memory: memory,
        }
    }
}
