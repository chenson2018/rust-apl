#![allow(rustdoc::private_intra_doc_links)]

//! This crate provides an interpreter for the language
//! [APL](https://en.wikipedia.org/wiki/APL_(programming_language))
//!
//! This crate constructs an interpreter for the APL programming language, written in Rust.
//! The primary inspiration is from Bob Nystrom's excellent [Crafting Interpreters](https://github.com/munificent/craftinginterpreters).
//! I also have used a significant amount of code from [Emily Signet's implementation of Lox](https://github.com/emily-signet/lax).
//!
//! The interpreter is relatively bare-bones. It supports a wide range of primitive glyphs,
//! variables, and [direct
//! functions](https://en.wikipedia.org/wiki/Direct_function#Dfns_versus_tradfns), but was created
//! mostly as an instructive exercise for myself and is thus missing many features a full
//! implementation would have. In this documentation I have tried to provide sufficient
//! explanations for anyone who might want to try something similar.
//!
//! ## Executable
//!
//! See [run.rs](crate::run::run) for the execution order and note how it is used
//! for the [interpreter executable](https://github.com/chenson2018/rust-apl/blob/main/src/main.rs).
//! The below sections attempt to give a guide for each piece.
//!
//! ## Scanning
//!
//! First, we enumerate the different types of tokens with [token::TokenType], a representation of
//! the groups of characters that have a syntactic meaning in our language. These can be a single
//! character, like '+', or a group of characters, like '1.25' or 'hello'. This process is called
//! scanning, performed in this crate by [scanner::Scanner] to produce a
//! vector of [token::Token].
//!
//! TODO: put an example
//!
//! ## Parsing
//!
//! Next, now that we have a series of token, we want to transform this into an expression. The key
//! here is that any expression can be represented by a single tree like structure, called an
//! [abstact syntax tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree). This process is
//! called parsing, performed in this crate by [parser::Parser] to produce a
//! vector of [expr::Expr]. Continuing with the previous example:
//!
//! TODO: put an example
//!
//! ## Evaluation
//!
//! TODO
//!
//! ## Variables
//!
//! TODO
//!
//! ## Functions
//!
//! TODO
//!

pub mod interpreter;
pub mod parser;
pub mod run;
pub mod scanner;

mod apl_type;
mod environment;
mod err;
mod expr;
mod primitives;
mod token;
