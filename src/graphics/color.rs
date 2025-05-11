#[derive(Clone, Copy)]
pub struct Color(pub u32);

impl Color {
    pub const BLACK: Self = Self::from_rgba(0x000000ff);
    pub const WHITE: Self = Self::from_rgba(0xffffffff);
    pub const RED: Self = Self::from_rgba(0xff0000ff);
    pub const GREEN: Self = Self::from_rgba(0x00ff00ff);
    pub const BLUE: Self = Self::from_rgba(0x0000ffff);

    pub const fn from_rgba(x: u32) -> Self {
        Self(x.swap_bytes())
    }
    pub const fn from_rgb(x: u32) -> Self {
        Self((x << 8 | 0xFF).swap_bytes())
    }

    pub const fn r(&self) -> u8 {
        self.0 as u8
    }
    pub const fn g(&self) -> u8 {
        (self.0 >> 8) as u8
    }
    pub const fn b(&self) -> u8 {
        (self.0 >> 16) as u8
    }
    pub const fn a(&self) -> u8 {
        (self.0 >> 24) as u8
    }
}
