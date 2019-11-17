#[derive(Clone, Copy)]
pub struct Format {
    pub channels: u8,
    pub sample_rate: u16,
    pub buffer_size: u16
}

impl Format {
    pub fn new() -> Self { Self {
        channels: 2,
        sample_rate: 44100,
        buffer_size: 512
    } }
}