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
    fn as_str(&self) -> &'static str {
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
        f.write_str(self.as_str())
    }
}

const RESET: &str = "\x1B[0m";

fn leaves_with_lights(leaf: char, leaf_count: usize, colors: &[TermColor], percent: f64) -> String {
    let light_count = (leaf_count as f64 * percent).floor() as usize;

    let mut rng = rand::rng();
    let indices = rand::seq::index::sample(&mut rng, leaf_count, light_count)
        .iter()
        .collect::<HashSet<_>>();

    let green = TermColor::Green.as_str();
    let mut buf = String::with_capacity(green.len() + leaf_count);
    buf.push_str(green);

    for i in 0..leaf_count {
        if indices.contains(&i)
            && let Some(color) = colors.choose(&mut rng)
        {
            buf.push_str(color.as_str());
            buf.push(leaf);
            buf.push_str(green);
        } else {
            buf.push(leaf);
        }
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
        padding = cli.level + 1
    );

    {
        let percent = f64::from(cli.percent) / 100.0;
        let spaces = " ".repeat(cli.level);

        for count in (1..=cli.level).map(|n| n * 2 + 1) {
            let padding = &spaces[..cli.level - count / 2];
            let leaves = leaves_with_lights(cli.char, count, &colors[..], percent);
            println!("{padding}{leaves}{RESET}");
        }
    }

    for _ in 0..(cli.level / 5).max(1) {
        println!(
            // RGB for brown
            "\x1B[38;2;222;119;51m{trunc:>padding$}{RESET}",
            trunc = '|',
            padding = cli.level + 1,
        );
    }
}
