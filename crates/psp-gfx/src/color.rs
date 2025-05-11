#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct Color32(u32);

impl Color32 {
    pub const BLACK: Self = Self::from_rgba(0x000000ff);
    pub const WHITE: Self = Self::from_rgba(0xffffffff);
    pub const RED: Self = Self::from_rgba(0xff0000ff);
    pub const GREEN: Self = Self::from_rgba(0x00ff00ff);
    pub const BLUE: Self = Self::from_rgba(0x0000ffff);
    pub const YELLOW: Self = Self::from_rgba(0xffff00ff);
    pub const CYAN: Self = Self::from_rgba(0x00ffffff);
    pub const MAGENTA: Self = Self::from_rgba(0xff00ffff);
    pub const PURPLE: Self = Self::from_rgba(0x800080ff);
    pub const ORANGE: Self = Self::from_rgba(0xffa500ff);
    pub const BROWN: Self = Self::from_rgba(0xa52a2aff);
    pub const PINK: Self = Self::from_rgba(0xffc0cbff);
    pub const GRAY: Self = Self::from_rgba(0x808080ff);
    pub const LIGHT_GRAY: Self = Self::from_rgba(0xd3d3d3ff);
    pub const DARK_GRAY: Self = Self::from_rgba(0x404040ff);
    pub const TRANSPARENT: Self = Self::from_rgba(0x00000000);

    pub const fn from_rgba(x: u32) -> Self {
        Self(x.swap_bytes())
    }
    pub const fn from_abgr(x: u32) -> Self {
        Self(x)
    }
    pub const fn from_rgb(x: u32) -> Self {
        Self((x << 8 | 0xFF).swap_bytes())
    }

    pub const fn as_rgba(&self) -> u32 {
        self.0.swap_bytes()
    }
    pub fn as_abgr(&self) -> u32 {
        self.0
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
