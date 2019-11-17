
use crate::Format;

pub struct Buffer {
    buf: Vec<f64>,
    format: Format
}

impl Buffer {
    
    pub fn new(format: Format) -> Self { Self {
        buf: Vec::with_capacity(format.buffer_size as usize),
        format
    } }

    pub fn get_buf (&self) -> &Vec<f64> {
        &self.buf
    }

    pub fn get_mut_buf(&mut self) -> &mut Vec<f64> {
        &mut self.buf
    }

    pub fn get_format(&self) -> Format {
        self.format
    }

}

impl From<Format> for Buffer {
    fn from(format: Format) -> Self {
        Buffer::new(format)
    }
}