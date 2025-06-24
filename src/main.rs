use clap::Parser;
use rand::seq::IndexedRandom;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug)]
pub enum TermColor {
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Red,
    White,
    Yellow,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
}

impl TermColor {
    fn to_str(&self) -> &str {
        match self {
            Self::Black => "\x1B3[30m",
            Self::Blue => "\x1B[34m",
            Self::Cyan => "\x1B[36m",
            Self::Green => "\x1B[32m",
            Self::Magenta => "\x1B[35m",
            Self::Red => "\x1B[31m",
            Self::White => "\x1B[37m",
            Self::Yellow => "\x1B[33m",
            Self::LightBlack => "\x1B[90m",
            Self::LightBlue => "\x1B[94m",
            Self::LightCyan => "\x1B[96m",
            Self::LightGreen => "\x1B[92m",
            Self::LightMagenta => "\x1B[95m",
            Self::LightRed => "\x1B[91m",
            Self::LightWhite => "\x1B[97m",
            Self::LightYellow => "\x1B[93m",
        }
    }
}

impl Display for TermColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

const RESET: &str = "\x1B[0m";

fn nth_odd(n: usize) -> usize {
    n * 2 + 1
}

fn leaves_with_lights(leaf: char, leaf_count: usize, colors: &[TermColor], percent: f64) -> String {
    let light_count = (leaf_count as f64 * percent).floor() as usize;

    let mut rng = rand::rng();
    let indices = rand::seq::index::sample(&mut rng, leaf_count, light_count)
        .iter()
        .collect::<HashSet<_>>();

    let green = TermColor::Green.to_str();
    let mut buf = String::from(green);

    for i in 0..leaf_count {
        if indices.contains(&i) {
            if let Some(color) = colors.choose(&mut rng) {
                buf.push_str(color.to_str());
                buf.push(leaf);
                buf.push_str(green);
                continue;
            }
        }
        buf.push(leaf);
    }

    buf
}

#[derive(Parser)]
struct Cli {
    /// Number of leaf rows
    #[arg(short, long, default_value_t = 5)]
    level: usize,

    /// Character to use for leaves and lights
    #[arg(short, long, default_value_t = '@')]
    char: char,

    /// Percent of leaves to replace with lights (0 for no lights)
    #[arg(short, long, default_value_t = 50, value_parser = clap::value_parser!(u8).range(0..=100))]
    percent: u8,
}

fn main() {
    let cli = Cli::parse();

    let padding = nth_odd(cli.level) / 2;
    let colors = [
        TermColor::Yellow,
        TermColor::Red,
        TermColor::Blue,
        TermColor::Magenta,
        TermColor::Cyan,
        TermColor::White,
    ];

    println!(
        "{color}{star:>padding$}{RESET}",
        color = TermColor::LightYellow,
        star = '*',
        padding = padding + 1
    );

    let percent = cli.percent as f64 / 100.0;
    for count in (1..=cli.level).map(nth_odd) {
        let leftpad = " ".repeat(padding - (count / 2));
        let leaves = leaves_with_lights(cli.char, count, &colors[..], percent);
        println!("{leftpad}{leaves}{RESET}");
    }

    for _ in 0..(cli.level / 5).max(1) {
        println!(
            // RGB for brown
            "\x1B[38;2;222;119;51m{trunc:>padding$}{RESET}",
            trunc = '|',
            padding = padding + 1,
        );
    }
}
