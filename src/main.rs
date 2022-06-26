//use std::path::PathBuf;
use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::error::Error;
use std::fs::File;
use std::path::Path;

use rust_apl::interpreter::Interpreter;
use rust_apl::run::run;
use std::io::Read;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
//use codespan_reporting::term::emit;

use rust_apl::err::AplErrors;

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
            let mut file = match File::open(&path) {
                // The `description` method of `io::Error` returns a string that describes the error
                Err(why) => panic!(
                    "couldn't open {}: {}",
                    display,
                    <dyn Error>::to_string(&why)
                ),
                Ok(file) => file,
            };

            let mut buffer = String::new();
            file.read_to_string(&mut buffer).unwrap();

            match run(buffer, &mut interpreter, args.verbose) {
                Ok(vals) => vals.iter().map(|x| println!("{}", x)).collect(),
                Err(errs) => println!("{}", errs),
            }
        }

        // otherwise enter an interactive session
        None => {
            let mut rl = Editor::<()>::new();
            let mut files = SimpleFiles::new();

            loop {
                let readline = rl.readline("> ");

                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        match run(format!("{}\n", line), &mut interpreter, args.verbose) {
                            Ok(vals) => vals.iter().map(|x| println!("{}", x)).collect(),
                            Err(errs) => {
                                let file_id = files.add("REPL", line.clone());

                                match errs {
                                    AplErrors(v) => {
                                        for e in v {
                                            let diagnostic = Diagnostic::error()
                                                .with_message(e.message)
                                                .with_labels(vec![Label::primary(
                                                    file_id,
                                                    (e.start)..(e.end),
                                                )
                                                .with_message(e.label)])
                                                .with_notes(vec![e.err]);

                                            let writer =
                                                StandardStream::stderr(ColorChoice::Always);
                                            let config =
                                                codespan_reporting::term::Config::default();
                                            codespan_reporting::term::emit(
                                                &mut writer.lock(),
                                                &config,
                                                &files,
                                                &diagnostic,
                                            )
                                            .unwrap();
                                        }
                                    }
                                }
                            }
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
