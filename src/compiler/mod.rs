use std::fs::File;
use std::io::Read;
use thiserror::Error;

use crate::compiler::lexer::LexerError;

pub mod lexer;
pub mod parser;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("The compiler has run into an issue while reading a file.")]
    IOError(#[from] std::io::Error),
    #[error("The compiler has run into an issue while scanning the file for lexemes.")]
    LexerError,
}

pub fn compile(path: &str) -> Result<String, CompilerError> {
    let f = File::open(path)?;
    let mut peekable = f
        .bytes()
        .map(|a| char::from(a.unwrap_or_default()))
        .peekable();
    let result = lexer::scan(&mut peekable);

    match result {
        Ok(ok_result) => {
            println!("{:?}", &ok_result);
            // TODO: Replace the return value with the final compiled string.
            unimplemented!()
        }
        Err(err) => {
            println!("Compilation failed: {}", &err);
            Err(CompilerError::LexerError)
        }
    }
}
