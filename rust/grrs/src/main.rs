use ansi_term::{Color, Style};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args: Cli = Cli::from_args();
    let p = &args.pattern;

    let reader: Box<dyn BufRead> = if let Some(path) = &args.path {
        Box::new(BufReader::new(File::open(path)?))
    } else {
        Box::new(BufReader::new(std::io::stdin()))
    };

    for line in reader.lines() {
        let line = line?;
        if line.contains(p) {
            let line = line.replace(p, &Color::Red.bold().paint(p).to_string());
            println!("{}", line);
        }
    }
    Ok(())
}
