use fluent_ansi::color::RGBColor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBf64 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RGBf64 {
    pub fn to_rgb_color(self) -> RGBColor {
        fn to_u8(component: f64) -> u8 {
            #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let result = (component * 255.0).round() as u8;
            result
        }

        RGBColor::new(to_u8(self.r), to_u8(self.g), to_u8(self.b))
    }
}

impl From<RGBColor> for RGBf64 {
    fn from(color: RGBColor) -> Self {
        fn from_u8(component: u8) -> f64 {
            f64::from(component) / 255.0
        }

        Self {
            r: from_u8(color.r),
            g: from_u8(color.g),
            b: from_u8(color.b),
        }
    }
}
