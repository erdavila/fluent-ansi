use std::ops::Mul;

use fluent_ansi::{
    Formatted,
    color::{EightBitColor, RGBColor},
    prelude::*,
};

fn main() {
    flags();
    println!();
    simple_colors();
    println!();
    eight_bit_colors();
    println!();
    rgb();
    println!();
}

fn flags() {
    print_title("Flags");
    let flags = [
        (Flag::Bold, "Bold"),
        (Flag::Faint, "Faint"),
        (Flag::Italic, "Italic"),
        (Flag::Underline, "Underline"),
        (Flag::SlowBlink, "SlowBlink"),
        (Flag::RapidBlink, "RapidBlink"),
        (Flag::Reverse, "Reverse"),
        (Flag::Conceal, "Conceal"),
        (Flag::CrossedOut, "CrossedOut"),
        (Flag::DoubleUnderline, "DoubleUnderline"),
        (Flag::Overline, "Overline"),
    ];
    for (flag, name) in flags {
        println!("{} {name}", flag.applied_to("Sample"));
    }
}

fn simple_colors() {
    print_title("Simple Colors");
    let colors = [
        (Color::BLACK, "Black"),
        (Color::RED, "Red"),
        (Color::GREEN, "Green"),
        (Color::YELLOW, "Yellow"),
        (Color::BLUE, "Blue"),
        (Color::MAGENTA, "Magenta"),
        (Color::CYAN, "Cyan"),
        (Color::WHITE, "White"),
    ];
    print_subtitle("Basic");
    for (color, name) in colors {
        println!(
            "{} {} {name}",
            color.in_fg().applied_to("Foreground"),
            color.in_bg().applied_to("Background")
        );
    }
    print_subtitle("Bright");
    for (color, name) in colors {
        println!(
            "{} {} Bright {name}",
            color.bright().in_fg().applied_to("Foreground"),
            color.bright().in_bg().applied_to("Background")
        );
    }
}

fn eight_bit_colors() {
    print_title("Eight Bit Colors");
    for index in 0..=255u8 {
        let content = format!(" {index:3} ");
        let formatted = EightBitColor(index).in_bg().applied_to(content);
        print!("{formatted}");
        match index {
            7 | 15 | 51 | 87 | 123 | 159 | 195 | 231 | 255 => println!(),
            _ => {}
        }
    }
}

fn rgb() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Rgb {
        r: u8,
        g: u8,
        b: u8,
    }
    impl Rgb {
        const BLACK: Rgb = Rgb::new(0, 0, 0);
        const RED: Rgb = Rgb::new(255, 0, 0);
        const YELLOW: Rgb = Rgb::new(255, 255, 0);
        const GREEN: Rgb = Rgb::new(0, 255, 0);
        const CYAN: Rgb = Rgb::new(0, 255, 255);
        const BLUE: Rgb = Rgb::new(0, 0, 255);
        const MAGENTA: Rgb = Rgb::new(255, 0, 255);
        const WHITE: Rgb = Rgb::new(255, 255, 255);

        const fn new(r: u8, g: u8, b: u8) -> Self {
            Self { r, g, b }
        }
    }
    impl Mul<f64> for Rgb {
        type Output = Rgb;

        #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        fn mul(self, rhs: f64) -> Self::Output {
            let r = (f64::from(self.r) * rhs) as u8;
            let g = (f64::from(self.g) * rhs) as u8;
            let b = (f64::from(self.b) * rhs) as u8;
            Rgb::new(r, g, b)
        }
    }

    struct RGBInterpolator {
        r: U8Interpolator,
        g: U8Interpolator,
        b: U8Interpolator,
    }
    impl RGBInterpolator {
        fn new(from: Rgb, to: Rgb, steps: u8) -> Self {
            Self {
                r: U8Interpolator::new(from.r, to.r, steps),
                g: U8Interpolator::new(from.g, to.g, steps),
                b: U8Interpolator::new(from.b, to.b, steps),
            }
        }
    }
    impl Iterator for RGBInterpolator {
        type Item = Rgb;

        fn next(&mut self) -> Option<Self::Item> {
            if let (Some(r), Some(g), Some(b)) = (self.r.next(), self.g.next(), self.b.next()) {
                Some(Rgb::new(r, g, b))
            } else {
                None
            }
        }
    }

    fn print_color(rgb: Rgb) {
        let hexcode = format!(" {:02x}{:02x}{:02x} ", rgb.r, rgb.g, rgb.b);
        let color = RGBColor::new(rgb.r, rgb.g, rgb.b);

        print!("{}", color.in_bg().applied_to(hexcode));
    }

    fn print_gradient(from: Rgb, to: Rgb, steps: u8) {
        let inter = RGBInterpolator::new(from, to, steps);
        for rgb in inter.take(usize::from(steps) - 1) {
            print_color(rgb);
        }
    }

    fn print_gradients(marks: impl IntoIterator<Item = Rgb>) {
        const STEPS: u8 = 5;
        let mut iter = marks.into_iter();

        if let Some(mut a) = iter.next() {
            for b in iter {
                print_gradient(a, b, STEPS);
                a = b;
            }

            print_color(a);
        }
    }

    print_title("RGB colors");
    let sequence = [
        Rgb::RED,
        Rgb::YELLOW,
        Rgb::GREEN,
        Rgb::CYAN,
        Rgb::BLUE,
        Rgb::MAGENTA,
        Rgb::RED,
    ];
    print_gradients(sequence);
    println!();
    print_gradients(sequence.map(|rgb| rgb * 0.75));
    println!();
    print_gradients(sequence.map(|rgb| rgb * 0.50));
    println!();
    print_gradients(sequence.map(|rgb| rgb * 0.25));
    println!();
    print_gradient(Rgb::BLACK, Rgb::WHITE, 26);
    println!();
}

fn print_title(title: &str) {
    println!("{}", Formatted::new(format!("=== {title} ===")).bold());
}

fn print_subtitle(subtitle: &str) {
    println!("{}", Formatted::new(format!("--- {subtitle} ---")).bold());
}

struct U8Interpolator {
    from: u8,
    to: u8,
    steps: u8,
    next_step: u8,
}
impl U8Interpolator {
    fn new(from: u8, to: u8, steps: u8) -> Self {
        Self {
            from,
            to,
            steps,
            next_step: 0,
        }
    }
}
impl Iterator for U8Interpolator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        (self.next_step < self.steps).then(|| {
            let step = self.next_step;
            self.next_step += 1;

            if self.steps == 1 {
                self.from
            } else {
                let from = i32::from(self.from);
                let to = i32::from(self.to);
                let step = i32::from(step);
                let steps = i32::from(self.steps);

                let n = from + step * (to - from) / (steps - 1);

                #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let n = n as u8;

                n
            }
        })
    }
}
