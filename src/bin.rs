use std::{fs, path::PathBuf};

use clap::Parser;
use tibrs::{compile, parse_str, say_hi};

#[derive(clap::Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    compile: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if let Some(path) = args.compile {
        let content = fs::read_to_string(path).unwrap();

        match parse_str(&content) {
            Err(e) => println!("{e}"),
            Ok(tokens) => fs::write("RESULT.8xp", compile(tokens).unwrap()).unwrap(),
        };
    }
}
