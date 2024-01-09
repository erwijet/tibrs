use std::{
    fs::{self, File},
    io::prelude::*,
    path::PathBuf,
};

use clap::Parser;
use tibrs::{compile, decompile, parse_str};

#[derive(clap::Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    compile: bool,

    #[arg(short, long)]
    decompile: bool,

    #[arg(short, long, required = true)]
    outfile: PathBuf,

    #[arg(required = true)]
    infile: PathBuf,
}

fn main() {
    let args = Args::parse();
    let outfile = args.outfile.clone();
    let infile = args.infile.clone();

    match args {
        Args { compile: true, .. } => {
            let path = outfile.clone();
            let outfile_name = path.file_stem().unwrap().to_str().unwrap().to_uppercase();

            match parse_str(&fs::read_to_string(infile).unwrap()) {
                Err(e) => eprintln!("{e}"),
                Ok(tokens) => {
                    fs::write(outfile, compile(tokens, outfile_name.as_bytes()).unwrap()).unwrap()
                }
            };
        }

        Args {
            decompile: true, ..
        } => {
            let path = outfile.clone();
            let inbuf = fs::read(infile).unwrap();

            if outfile.exists() {
                fs::remove_file(path).unwrap();
            }

            let mut f = File::create(outfile).unwrap();

            match decompile(inbuf.into_boxed_slice()) {
                Err(e) => eprintln!("{e}"),
                Ok(tokens) => {
                    tokens.iter().for_each(|tok| {
                        if tok.text == "@@NEWLINE" {
                            writeln!(f).unwrap();
                        } else {
                            write!(f, "{}", tok.text).unwrap();
                        }
                    });

                    f.flush().unwrap()
                }
            };
        }

        _ => {}
    }
}
