use crate::apl_type::AplType;
use crate::err::AplError;
use crate::interpreter::Interpreter;
use crate::parser::Parser as AplParser;
use crate::scanner::Scanner;
use std::io;
use std::io::Write;

pub fn run(s: String, i: &mut Interpreter, verbose: bool) -> Result<AplType, AplError> {
    io::stdout().flush().unwrap();
    let mut scanner = Scanner::new(s);

    match scanner.scan() {
        Ok(_) => (),
        Err(err) => println!("{:?}", err),
    }

    if verbose {
        for t in &scanner.tokens {
            println!("{:?}", t);
        }
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

            Ok(value)
        }
        Err(err) => Err(err),
    }
}
