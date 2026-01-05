use fluent_ansi::color::{Color, RGBColor};

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

    // According to WCAG 2.0 (Web Content Accessibility Guidelines)
    pub fn best_constrast_color(self) -> RGBColor {
        fn normalize(channel: f64) -> f64 {
            if channel <= 0.04045 {
                channel / 12.92
            } else {
                ((channel + 0.055) / 1.055).powf(2.4)
            }
        }

        let r = normalize(self.r);
        let g = normalize(self.g);
        let b = normalize(self.b);
        let luminance = r * 0.2126 + g * 0.7152 + b * 0.0722;

        let contrast_with_white = (1.0 + 0.05) / (luminance + 0.05);
        let contrast_with_black = (luminance + 0.05) / (0.0 + 0.05);

        if contrast_with_white > contrast_with_black {
            Color::rgb(255, 255, 255)
        } else {
            Color::rgb(0, 0, 0)
        }
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
