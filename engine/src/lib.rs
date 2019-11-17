
mod buffer;
pub use buffer::Buffer;

mod container;
pub use container::Container;

mod format;
pub use format::Format;

///
/// ticks are limited to 2^32 ticks
///     44100  - 1d 3h 3m 11s
///     192000 - 6h 12m 49s
/// 
pub struct Engine {
    ticks: u32,
    containers: Vec<Container>,
    format: Format
}

impl Engine {

    pub fn new(format: Format) -> Self { 
        Self {
            ticks: 0,
            containers: Vec::new(),
            format
        } 
    }

    pub fn next(&mut self) -> Buffer {
        let mut buf = Buffer::from(self.format);
        for container in self.containers.iter_mut() {
            container.fill_buffer(self.ticks, &mut buf);
        }
        self.update_ticks();
        buf
    }

    fn update_ticks(&mut self) {
        self.ticks = (self.format.buffer_size as u32) + self.ticks;
    }

}