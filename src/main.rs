#![allow(dead_code)] // Remove once the implementation looks OK

use rand::seq::IndexedRandom;
use std::{collections::HashSet, env, error::Error, fmt::Display};

#[derive(Debug)]
enum TermColor {
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
    fn to_str(&self) -> &'static str {
        match *self {
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

fn leaves_with_lights(leaf: char, leaf_count: usize, colors: &[TermColor]) -> String {
    let percent = 0.50; // TODO: customize
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

fn main() -> Result<(), Box<dyn Error>> {
    let levels: usize = match env::args().skip(1).next() {
        Some(arg) => arg.parse()?,
        None => 5,
    };

    let leaf = '@'; // TODO: customize
    let padding = nth_odd(levels) / 2 + 1;
    let colors = [
        TermColor::Yellow,
        TermColor::Red,
        TermColor::Blue,
        TermColor::Magenta,
        TermColor::Cyan,
    ];

    println!("{}{:>padding$}{RESET}", TermColor::LightYellow, "*");

    for count in (1..=levels).map(nth_odd) {
        let leftpad = " ".repeat(padding - (count / 2 + 1));
        let leaves = leaves_with_lights(leaf, count, &colors[..]);
        println!("{leftpad}{leaves}{RESET}");
    }

    {
        let brown = "\x1B[38;2;222;119;51m";
        for _ in 0..(levels / 5).max(1) {
            println!("{brown}{:>padding$}{RESET}", "|");
        }
    }

    Ok(())
}
