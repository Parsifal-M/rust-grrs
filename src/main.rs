#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()));
    let mut count = 0;

    for line in content.expect("REASON").lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
            count += 1;
        }
    }
    println!("Found {} times", count);
}
