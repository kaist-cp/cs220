use std::fs::File;
use std::io::{self, BufRead, Read};

use anyhow::Result;
use clap::Parser;

use cs220::assignments::assignment04::*;

struct Input<'a> {
    source: Box<dyn BufRead + 'a>,
}

impl<'a> Input<'a> {
    fn console(stdin: &'a io::Stdin) -> Input<'a> {
        Input {
            source: Box::new(stdin.lock()),
        }
    }

    fn file(path: &str) -> io::Result<Input<'a>> {
        File::open(path).map(|file| Input {
            source: Box::new(io::BufReader::new(file)),
        })
    }
}

impl<'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.source.read(buf)
    }
}

impl<'a> BufRead for Input<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.source.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.source.consume(amt);
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Expression filepath.
    #[clap(value_parser)]
    filepath: Option<String>,
}

fn main() -> Result<()> {
    // Parses arguments.
    let args = Args::parse();

    let stdin = io::stdin();
    let input = if let Some(filepath) = args.filepath {
        Input::file(&filepath)?
    } else {
        Input::console(&stdin)
    };

    let mut context = context::Context::new();
    for line in input.lines() {
        let command = parser::parse_command(&line?)?;
        let (variable, value) = context.calc_command(&command)?;
        println!("{} = {}", variable, value);
    }

    Ok(())
}
