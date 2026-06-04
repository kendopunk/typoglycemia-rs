/// Quick CLI harness for manual testing during development.
///
/// Usage:
///   cargo run --example try -- "your text here"
///   cargo run --example try -- --leet "your text here"
///   cargo run --example try -- --leet 2 "your text here"
///
/// Flags:
///   --leet [level]   Run typoglycemia_leet() at the given level (1–3, default 1)
///   (no flag)        Run typoglycemia()
use typoglycemia::{typoglycemia, typoglycemia_leet};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage:");
        eprintln!("  cargo run --example try -- \"your text here\"");
        eprintln!("  cargo run --example try -- --leet \"your text here\"");
        eprintln!("  cargo run --example try -- --leet 2 \"your text here\"");
        std::process::exit(1);
    }

    let mut remaining = args.as_slice();
    let mut leet_level: Option<u8> = None;

    if remaining.first().map(String::as_str) == Some("--leet") {
        remaining = &remaining[1..];
        // Check if the next arg is a level digit (1–3)
        let level = remaining
            .first()
            .and_then(|s| s.parse::<u8>().ok())
            .filter(|&n| (1..=3).contains(&n));
        if level.is_some() {
            remaining = &remaining[1..];
        }
        leet_level = Some(level.unwrap_or(1));
    }

    let input = remaining.join(" ");

    if input.is_empty() {
        eprintln!("Error: no input text provided.");
        std::process::exit(1);
    }

    let result = match leet_level {
        Some(level) => typoglycemia_leet(&input, level),
        None => typoglycemia(&input),
    };

    println!("Input:  {}", input);
    println!("Output: {}", result);
}
