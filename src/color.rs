use prisma::{Lerp, Rgb};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn lerp(&self, to: &Color, t: u64, max: f64) -> Color {
        let prisma_col = Rgb::new(self.r, self.g, self.b);
        let prisma_col_to = Rgb::new(to.r, to.g, to.b);

        let ret_col = prisma_col.lerp(&prisma_col_to, t as f64 / max);

        Color::from(ret_col.red(), ret_col.green(), ret_col.blue())
    }
}
