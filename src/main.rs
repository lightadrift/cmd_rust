#![allow(unused)]

use clap::Parser;
use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

// fn main() {
//     let args = Cli::parse();
//     let f = File::open(&args.path).expect("could not read file");
//     let mut reader = BufReader::new(f);
//     let mut buffer: Vec<u8> = Vec::new();
//     reader.read_to_end(&mut buffer);
//     let contents = String::from_utf8(buffer).unwrap();
//     for line in contents.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }
// }

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let f = File::open(&args.path)?;
    let mut reader = BufReader::new(f);
    let pattern_found = reader.lines().filter_map(Result::ok).any(|line| {
        if line.contains(&args.pattern) {
            println!("{}", line);
            true
        } else {
            false
        }
    });
    if !pattern_found {
        eprintln!("Pattern not found.");
    }
    Ok(())
}
