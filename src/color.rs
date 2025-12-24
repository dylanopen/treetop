use bevy_ecs::component::Component;

#[derive(Component)]
pub struct TreetopColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl TreetopColor {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_crossterm_color(&self) -> crossterm::style::Color {
        crossterm::style::Color::Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    pub const BLACK: TreetopColor = TreetopColor { r: 0, g: 0, b: 0 };
    pub const WHITE: TreetopColor = TreetopColor { r: 255, g: 255, b: 255 };
    pub const RED: TreetopColor = TreetopColor { r: 255, g: 0, b: 0 };
    pub const GREEN: TreetopColor = TreetopColor { r: 0, g: 255, b: 0 };
    pub const BLUE: TreetopColor = TreetopColor { r: 0, g: 0, b: 255 };
    pub const YELLOW: TreetopColor = TreetopColor { r: 255, g: 255, b: 0 };
    pub const CYAN: TreetopColor = TreetopColor { r: 0, g: 255, b: 255 };
    pub const MAGENTA: TreetopColor = TreetopColor { r: 255, g: 0, b: 255 };
    pub const GRAY: TreetopColor = TreetopColor { r: 128, g: 128, b: 128 };
    pub const ORANGE: TreetopColor = TreetopColor { r: 255, g: 165, b: 0 };
    pub const PURPLE: TreetopColor = TreetopColor { r: 128, g: 31, b: 255 };
    pub const PINK: TreetopColor = TreetopColor { r: 255, g: 162, b: 203 };
}
