use crate::apl_type::AplType;
use crate::err::AplErrors;
use crate::interpreter::Interpreter;
use crate::parser::Parser as AplParser;
use crate::scanner::Scanner;
use std::io;
use std::io::Write;

/// Given an [Interpreter](crate::interpreter::Interpreter), evaluate a given string.
pub fn run(s: String, i: &mut Interpreter, verbose: bool) -> Result<Vec<AplType>, AplErrors> {
    io::stdout().flush().unwrap();
    let mut scanner = Scanner::new(s);

    // Scanning
    // it's kinda weird that this doesn't return the tokens like the parser returns ast
    match scanner.scan() {
        Ok(_) => (),
        Err(err) => return Err(AplErrors(err)),
    }

    if verbose {
        println!("Tokens:\n");

        for t in &scanner.tokens {
            println!("\t{:?}", t);
        }

        println!();
    }

    // Parsing
    let mut parser = AplParser::new(scanner.tokens);
    let ast = parser.parse();

    match ast {
        Ok(_) => (),
        Err(err) => return Err(AplErrors(vec![err])),
    }

    let ast_verified = ast.unwrap();

    if verbose {
        println!("Rust AST: {:#?}\n", &ast_verified);
    }

    // Interpreting
    let value = i.interpret(&ast_verified);

    match value {
        Ok(_) => (),
        Err(err) => return Err(AplErrors(vec![err])),
    }

    let value_verified = value.unwrap();

    if verbose {
        println!("Evaluates to: {:#?}\n", value_verified);
    }

    Ok(value_verified)
}
