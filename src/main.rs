//use std::path::PathBuf;
use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::error::Error;
use std::fs::File;
//use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

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
    #[clap(short, long, action, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let mut interpreter = Interpreter::new();

    match args.path {
        // if a path is provided, execute that file
        // currently only works for a single line!
        Some(p) => {
            // Create a path to the desired file
            let path = Path::new(&p);
            let display = path.display();

            // Open the path in read-only mode, returns `io::Result<File>`
            let file = match File::open(&path) {
                // The `description` method of `io::Error` returns a string that describes the error
                Err(why) => panic!(
                    "couldn't open {}: {}",
                    display,
                    <dyn Error>::to_string(&why)
                ),
                Ok(file) => file,
            };

            // Collect all lines into a vector
            let reader = BufReader::new(file);
            let lines: Vec<_> = reader.lines().collect();

            for l in lines {
                // there HAS to be a better way to read and keep new lines
                let mut s = l.unwrap();
                s.push('\n');

                match run(s, &mut interpreter, args.verbose) {
                    Ok(value) => println!("{}", value),
                    Err(err) => println!("{}", err),
                };
            }
        }

        // otherwise enter an interactive session
        None => {
            let mut rl = Editor::<()>::new();
            loop {
                let readline = rl.readline("> ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        match run(format!("{}\n", line), &mut interpreter, args.verbose) {
                            Ok(value) => println!("{}", value),
                            Err(errs) => println!("{}", errs),
                        }
                    }
                    Err(ReadlineError::Interrupted) => {
                        println!("CTRL-C");
                        break;
                    }
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
            }
        }
    }
}
