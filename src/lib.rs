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
//! mostly as an instructive exercise for myself. In the sections below I have tried to provide
//! sufficient documentation for anyone who might want to try something similar.
//!
//! ## Overview
//!
//! To see the order that the interpreter uses, see [run.rs](crate::run::run) and how it is used
//! for the [interpreter executable](../src/rust_apl/main.rs.html). The below sections attempt to
//! give a guide for each piece.
//!
//! ### Scanning
//!
//! First, we enumerate the different types of tokens with [token::TokenType].
//!
//! ### Parsing
//!
//! TODO
//!
//! ### Evaluation
//!
//! TODO
//!
//! ### Variables
//!
//! TODO
//!
//! ### Functions
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
