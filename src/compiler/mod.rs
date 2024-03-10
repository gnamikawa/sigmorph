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
    #[error("The compiler has run into an issue while scanning.")]
    LexerError,
}

pub fn compile(path: &str) -> Result<(), CompilerError> {
    let f = File::open(path)?;
    let mut peekable = f
        .bytes()
        .map(|a| char::from(a.unwrap_or_default()))
        .peekable();
    let result = lexer::scan(&mut peekable);
    match result {
        Ok(ok_result) => {
            println!("{:?}", ok_result);
            return Ok(());
        }
        Err(err) => {
            println!("Compilation failed: {}", err);
            return Err(CompilerError::LexerError);
        }
    }
}

#[test]
fn compile_testcase_emptyfile() {
    let out = compile("assets/testcase_only-newline.sig");
    assert!(!out.is_err())
}

#[test]
fn compile_testcase_onlynewline() {
    let out = compile("assets/testcase_only-newline.sig");
    assert!(!out.is_err())
}

#[test]
fn compile_testcase_singleevent() {
    let out = compile("assets/testcase_single-event.sig");
    assert!(!out.is_err())
}

#[test]
fn compile_testcase_trailingnewline() {
    let out = compile("assets/testcase_trailing-newline.sig");
    assert!(!out.is_err())
}

#[test]
fn compile_testcase_singlelet() {
    let out = compile("assets/testcase_single-let.sig");
    assert!(!out.is_err())
}

#[test]
fn compile_testcase_singleargumentfunction() {
    let out = compile("assets/testcase_single-argument-function.sig");
    assert!(!out.is_err())
}

#[test]
fn compile_testcase_statesandtransitions() {
    let out = compile("assets/testcase_states-and-transitions.sig");
    assert!(!out.is_err())
}

#[test]
fn compile_testcase_comment() {
    let out = compile("assets/testcase_comment.sig");
    assert!(!out.is_err())
}
