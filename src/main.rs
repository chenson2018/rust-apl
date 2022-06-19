//use std::path::PathBuf;
use clap::Parser;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

use rust_apl::interpreter::Interpreter;
use rust_apl::run::run;

// this struct defines our command line arguments
/// A Rust Implementation of APL
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to APL script. If none, enter interactive session.
    #[clap(value_parser)]
    path: Option<String>,

    /// Print Interpreter Debugging
    #[clap(short, long, action, default_value_t = true)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let mut interpreter = Interpreter::new();

    match args.path {
        // if a path is provided, execute that file
        // currently only works for a single line!
        Some(p) => {
            let mut buffer = String::new();
            File::open(p).unwrap().read_to_string(&mut buffer).unwrap();
            match run(buffer, &mut interpreter, args.verbose) {
                Ok(value) => println!("{}", value),
                Err(err) => println!("{}", err),
            };
        }

        // otherwise enter an interactive session
        None => loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            match run(line, &mut interpreter, args.verbose) {
                Ok(value) => println!("{}", value),
                Err(err) => println!("{}", err),
            };
        },
    }
}
