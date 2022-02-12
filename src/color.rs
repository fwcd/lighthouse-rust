pub const BLACK: Color = Color { red: 0, green: 0, blue: 0 };
pub const WHITE: Color = Color { red: 255, green: 255, blue: 255 };
pub const RED: Color = Color { red: 255, green: 0, blue: 0 };
pub const GREEN: Color = Color { red: 0, green: 255, blue: 0 };
pub const BLUE: Color = Color { red: 0, green: 0, blue: 255 };
pub const YELLOW: Color = Color { red: 255, green: 255, blue: 0 };
pub const CYAN: Color = Color { red: 0, green: 255, blue: 255 };
pub const MAGENTA: Color = Color { red: 255, green: 0, blue: 255 };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}