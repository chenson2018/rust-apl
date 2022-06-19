//use std::path::PathBuf;
use clap::Parser;
use std::io;
use std::io::Write;
use std::fs::File;
use std::io::Read;

use rust_apl::scanner::Scanner;
use rust_apl::interpreter::Interpreter;
use rust_apl::parser::Parser as AplParser;

//temp
//use rust_apl::expr::Expr;
//use rust_apl::token::{Token,AplType};
//use rust_apl::token_type::TokenType;
//use std::rc::Rc;

//use itertools::Itertools; 

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
        run(buffer, &mut interpreter, args.verbose)
      },

      // otherwise enter an interactive session
      None => 
        loop {
          print!("> ");
          io::stdout().flush().unwrap();
          let mut line = String::new();
          io::stdin().read_line(&mut line).unwrap();
          run(line, &mut interpreter, args.verbose);
        }
      }
}

fn run(s: String, i: &mut Interpreter, verbose: bool) {
  io::stdout().flush().unwrap();
  let mut scanner = Scanner::new(s);

  match scanner.scan() {
       Ok(_) => (),
    Err(err) => println!("{:?}", err),
  }

  let mut parser = AplParser::new(scanner.tokens);
  let ast = parser.parse();

  // these errors aren't propogated up correctly yet, see the error handling I did for monadics...
  match ast {
    Ok(ast) => {
       // assuming a value for right now, really this will be () that prints is not an assignment
       let value = i.interpret(&ast).unwrap();

       // interpreter debugging
       if verbose {     
          println!("Polish notation: {}\n", &ast);
          println!("Rust AST: {:?}\n", &ast);
          println!("Evaluates to: {:?}\n", value);
       }

       // regular output
       println!("{}\n", value);
    },
    Err(err) => println!("{:?}", err),
  }
}
