
use crate::Buffer;

pub struct Container {
    start: u32,
    end: u32
    // inputs: Vec<dyn Inputs>
}

impl Container {

    pub fn new(start: u32, duration: u32) -> Self {
        Self {
            start,
            end: start + duration
        }
    }

    pub fn fill_buffer(&mut self, ticks: u32, buf: &mut Buffer) {
        // fill the buffer with inputs
    }
}