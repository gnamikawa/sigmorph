use std::fs::File;
use std::io::{Error, ErrorKind, Read};

use crate::compiler::lexer::LexerError;

pub mod lexer;
pub mod parser;

#[derive(Debug)]
pub enum CompilerError {
    IO(ErrorKind),
    LexerError,
}
impl From<Error> for CompilerError {
    fn from(error: Error) -> Self {
        CompilerError::IO(error.kind())
    }
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
            let message = match err {
                LexerError::IO(io_err) => format!("{:?}", io_err),
                compiler_err => format!("{:?}", compiler_err),
            };
            println!("Compilation failed: {:?}", message);
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
