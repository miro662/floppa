#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl From<wgpu::Color> for Color {
    fn from(color: wgpu::Color) -> Self {
        Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

impl From<Color> for wgpu::Color {
    fn from(color: Color) -> Self {
        wgpu::Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

#[allow(dead_code)]
pub const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
#[allow(dead_code)]
pub const GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};
#[allow(dead_code)]
pub const BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 1.0,
    a: 1.0,
};
#[allow(dead_code)]
pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
#[allow(dead_code)]
pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
#[allow(dead_code)]
pub const YELLOW: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};
