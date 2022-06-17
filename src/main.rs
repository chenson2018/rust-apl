//use std::path::PathBuf;
use clap::Parser;
use std::io;
use std::io::Write;
use std::fs::File;
use std::io::Read;

use rust_apl::scanner::Scanner;

// this struct defines our command line arguments
/// A Rust Implementation of APL
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to APL script. If none, enter interactive session.
    #[clap(value_parser)]
    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.path {
      // if a path is provided, execute that file
      Some(p) => {
        let mut buffer = String::new();
        File::open(p).unwrap().read_to_string(&mut buffer).unwrap();
        run(buffer)
      },

      // otherwise enter an interactive session
      None => 
        loop {
          print!("> ");
          io::stdout().flush().unwrap();
          let mut line = String::new();
          io::stdin().read_line(&mut line).unwrap();
          run(line);
        }
      }
}

fn run(s: String) {
  io::stdout().flush().unwrap();
  let mut scanner = Scanner::new(s);
  let scan_res = scanner.scan();

  match scan_res {
    Err(err) => println!("{:?}", err),
//    Ok(_)    => println!("{:?}", &scanner.tokens),
    Ok(_)    => { 
                  for t in &scanner.tokens {
                    println!("{:?}", t);
                  }
                },
  }

}
