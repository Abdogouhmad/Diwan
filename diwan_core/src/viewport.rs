use crate::buffer::Buffer;

pub struct Viewport {
    pub buffer: Buffer,
    pub top: u16,
    pub left: u16,
    pub width: u16,
    pub height: u16,
}

impl Viewport {
    pub fn new(buffer: Buffer, top: u16, left: u16, width: u16, height: u16) -> Viewport {
        Viewport {
            buffer,
            top,
            left,
            width,
            height,
        }
    }
}
