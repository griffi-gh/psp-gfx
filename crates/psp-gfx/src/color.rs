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

    /// Create a new [`Color32`] from an integer in the R8G8B8A8 format
    pub const fn from_rgba(x: u32) -> Self {
        Self(x.swap_bytes())
    }
    /// Create a new [`Color32`] from an integer in the A8B8G8R8 format (Native PSP)
    pub const fn from_abgr(x: u32) -> Self {
        Self(x)
    }
    /// Create a new [`Color32`] from an integer in the R8G8B8 format
    pub const fn from_rgb(x: u32) -> Self {
        Self((x << 8 | 0xFF).swap_bytes())
    }

    /// Get the color in the R8G8B8A8 format
    pub const fn as_rgba(&self) -> u32 {
        self.0.swap_bytes()
    }
    /// Get the color in the A8B8G8R8 format (Native PSP)
    pub fn as_abgr(&self) -> u32 {
        self.0
    }

    /// Get the Red component of the color
    pub const fn r(&self) -> u8 {
        self.0 as u8
    }
    /// Get the Green component of the color
    pub const fn g(&self) -> u8 {
        (self.0 >> 8) as u8
    }
    /// Get the Blue component of the color
    pub const fn b(&self) -> u8 {
        (self.0 >> 16) as u8
    }
    /// Get the Alpha component of the color
    pub const fn a(&self) -> u8 {
        (self.0 >> 24) as u8
    }
}
