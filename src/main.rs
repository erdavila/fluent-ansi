#![warn(clippy::pedantic)]

use fluent_ansi::{
    Styled,
    color::{EightBitColor, RGBColor},
    prelude::*,
};
use terminal_size::{Height, Width, terminal_size};

use crate::interpolation::from;
use crate::rgb_f64::RGBf64;

mod interpolation;
mod rgb_f64;

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

#[expect(clippy::similar_names)]
fn rgb() {
    fn print_color(color: RGBf64) {
        let color = color.to_rgb_color();
        let hexcode = format!(" {:02x}{:02x}{:02x} ", color.r, color.g, color.b);
        print!("{}", color.in_bg().applied_to(hexcode));
    }

    const CELL_WIDTH: u16 = 8; // 6 for hexcode + 2 for padding
    const COLOR_LINES: u16 = 7; // Prefer odd number for a unshaded line in the middle

    const BLACK: RGBColor = Color::rgb(0, 0, 0);
    const RED: RGBColor = Color::rgb(255, 0, 0);
    const YELLOW: RGBColor = Color::rgb(255, 255, 0);
    const GREEN: RGBColor = Color::rgb(0, 255, 0);
    const CYAN: RGBColor = Color::rgb(0, 255, 255);
    const BLUE: RGBColor = Color::rgb(0, 0, 255);
    const MAGENTA: RGBColor = Color::rgb(255, 0, 255);
    const WHITE: RGBColor = Color::rgb(255, 255, 255);

    let color_cycle = [RED, YELLOW, GREEN, CYAN, BLUE, MAGENTA];
    let range_count = color_cycle.len();

    #[expect(clippy::cast_precision_loss)]
    let range_count_f64 = range_count as f64;

    let (Width(w), _) = terminal_size().unwrap_or((Width(7 * CELL_WIDTH), Height(COLOR_LINES + 1)));
    let cells_per_line = w / CELL_WIDTH;

    let n_from_cell_index = |cell_index: u16| {
        from(0.0, cells_per_line - 1)
            .to(0.0, 1.0)
            .interpolate(cell_index)
    };

    print_title("RGB colors");

    // Color lines
    for line in 0..COLOR_LINES {
        // line == -1 would be all WHITE
        // line == COLOR_LINES would be all BLACK
        let shade_n = from(-1, COLOR_LINES).to(0.0, 1.0).interpolate(line);

        let (shade_range_n, shade_color) = if shade_n < 0.5 {
            let shade_range_n = from(0.0, 0.5).to(1.0, 0.0).interpolate(shade_n);
            (shade_range_n, WHITE)
        } else {
            let shade_range_n = from(0.5, 1.0).to(0.0, 1.0).interpolate(shade_n);
            (shade_range_n, BLACK)
        };

        for cell in 0..cells_per_line {
            let n = n_from_cell_index(cell);

            #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let range_index = ((n * range_count_f64) as usize).clamp(0, range_count - 1);

            let range_n = {
                #[expect(clippy::cast_precision_loss)]
                let range_index_f64 = range_index as f64;

                let range_start = range_index_f64 / range_count_f64;
                let range_end = (range_index_f64 + 1.0) / range_count_f64;
                from(range_start, range_end).to(0.0, 1.0).interpolate(n)
            };

            let color = {
                let from_color = color_cycle[range_index];
                let to_color = color_cycle[(range_index + 1) % range_count];
                from(0.0, 1.0)
                    .to_color(from_color, to_color)
                    .interpolate(range_n)
            };

            let shaded_color = from(0, 1.0)
                .to_color(color, shade_color)
                .interpolate(shade_range_n);

            print_color(shaded_color);
        }
        println!();
    }

    // Grayscale line
    for cell in 0..cells_per_line {
        let n = n_from_cell_index(cell);
        let color = from(0.0, 1.0).to_color(BLACK, WHITE).interpolate(n);
        print_color(color);
    }
}

fn print_title(title: &str) {
    println!("{}", Styled::new(format!("=== {title} ===")).bold());
}

fn print_subtitle(subtitle: &str) {
    println!("{}", Styled::new(format!("--- {subtitle} ---")).bold());
}
