// START -> DECLARATION | ASSIGNMENT;
// DECLARATION -> EVENT
// ASSIGNMENT -> const IDENTIFIER =
// IDENTIFIER

use rstest::rstest;
use thiserror::Error;

pub mod comments;

pub type Lexeme = String;

#[derive(Debug)]
pub enum TokenType {
    IDENTIFIER,
    KEYWORD,
    SEMICOLON,
    ASSIGNMENT,
    NUMERIC_LITERAL,
    EXPRESSION,
}

#[derive(Debug)]
pub struct TokenMetadata {
    line_number: i32,
    char_number: i32,
}

#[derive(Debug)]
pub struct Token {
    lexeme: Lexeme,
    token_type: Option<TokenType>,
    metadata: TokenMetadata,
}
#[derive(Debug, Error)]
pub enum LexerError {
    #[error("IO error")]
    IO(std::io::ErrorKind),
    #[error("Unexpected end of file")]
    UnexpectedEOF,
    #[error("Unexpected lexeme [{0}]")]
    UnexpectedLexeme(String),
    #[error("Unexpected declaration")]
    UnexpectedDeclaration,
}
impl From<std::io::Error> for LexerError {
    fn from(error: std::io::Error) -> Self {
        LexerError::IO(error.kind())
    }
}

#[rstest]
#[should_panic]
#[case::onlynewline("assets/testcase_only-newline.sig")]
#[should_panic]
#[case::emptyfile("assets/testcase_empty-file.sig")]
#[case::comment("assets/testcase_comment.sig")]
#[case::singleevent("assets/testcase_single-event.sig")]
#[case::trailingnewline("assets/testcase_trailing-newline.sig")]
#[case::singlelet("assets/testcase_single-let.sig")]
#[case::statesandtransitions("assets/testcase_states-and-transitions.sig")]
#[case::singleargumentfunction("assets/testcase_single-argument-function.sig")]
fn test_scan(#[case] test_case_path: &str) {
    use std::fs::File;
    use std::io::Read;
    let maybe_test_case = File::open(test_case_path);

    match maybe_test_case {
        Ok(test_case) => {
            let mut peekable = test_case
                .bytes()
                .map(|a| char::from(a.unwrap_or_default()))
                .peekable();

            let result = scan(&mut peekable);

            assert!(result.is_ok())
        }
        Err(_err) => {
            panic!();
        }
    }
}

/// Lexical analyzer. Takes tokenized input and generates an abstract syntax tree.
pub fn scan<I: Iterator<Item = char>>(
    iter: &mut std::iter::Peekable<I>,
) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    let result = document(iter, &mut tokens);
    match result {
        Ok(_) => {
            Ok(tokens)
        }
        Err(err) => {
            Err(err)
        }
    }
}

/// Consumes the lexeme from the iterator. Assigns a token if specified.
///
/// # Errors
///
/// This function will return an error if the expected lexeme could not be matched.
fn expect<I: Iterator<Item = char>>(
    iter: &mut std::iter::Peekable<I>,
    lexeme: &str,
    token_type: Option<TokenType>,
) -> Result<Token, LexerError> {
    for x in lexeme.chars() {
        if iter.next().unwrap_or_default() != x {
            return Err(LexerError::UnexpectedLexeme(String::from("asdf")));
        }
    }

    return Ok(Token {
        token_type,
        lexeme: lexeme.to_string(),
        metadata: TokenMetadata {
            line_number: 0, //TODO: Track line number
            char_number: 0, //TODO: Track char number
        },
    });
}

/// The starting and ending of a document.
fn document<I: Iterator<Item = char>>(
    iter: &mut std::iter::Peekable<I>,
    tokens: &mut Vec<Token>,
) -> Result<(), LexerError> {
    scope(iter, tokens)?;
    return Ok(());
}

/// Matches any scoped assignments, declarations, types and etc that are limited to a scope.
fn scope<I: Iterator<Item = char>>(
    iter: &mut std::iter::Peekable<I>,
    tokens: &mut Vec<Token>,
) -> Result<(), LexerError> {
    loop {
        match iter.peek() {
            Some(c) => match c {
                _ | '/' => {
                    whitespace(iter)?;
                    return Ok(());
                }
            },
            None => return Err(LexerError::UnexpectedEOF),
        }
    }
}

fn whitespace<I: Iterator<Item = char>>(
    iter: &mut std::iter::Peekable<I>,
) -> Result<(), LexerError> {
    loop {
        match iter.peek() {
            Some(c) => match c {
                c if c.is_whitespace() => {
                    iter.next();
                    whitespace(iter)?;
                }
                '/' => {
                    comments::comment(iter)?;
                }
                _ => {
                    return Ok(());
                }
            },
            None => return Err(LexerError::UnexpectedEOF),
        }
    }
}

fn declaration<I: Iterator<Item = char>>(
    iter: &mut std::iter::Peekable<I>,
    tokens: &mut Vec<Token>,
) -> Result<(), LexerError> {
    loop {
        match iter.peek() {
            Some('e') => {
                tokens.push(expect(iter, "event", Some(TokenType::KEYWORD))?);
                whitespace(iter)?;
                identifier(iter, tokens)?;
                tokens.push(expect(iter, ";", Some(TokenType::SEMICOLON))?);
                return Ok(());
            }
            Some('l') => {
                tokens.push(expect(iter, "let", Some(TokenType::KEYWORD))?);
                whitespace(iter)?;
                identifier(iter, tokens)?;
                whitespace(iter)?;
                tokens.push(expect(iter, "=", Some(TokenType::ASSIGNMENT))?);
                whitespace(iter)?;
                tokens.push(expect(iter, "100", Some(TokenType::NUMERIC_LITERAL))?); // TODO: Actually
                                                                                     // implement
                                                                                     // expressions
                whitespace(iter)?;
                tokens.push(expect(iter, ";", Some(TokenType::SEMICOLON))?);
                return Ok(());
            }
            _ => {
                return Err(LexerError::UnexpectedDeclaration);
            }
        }
    }
}

fn identifier<I: Iterator<Item = char>>(
    iter: &mut std::iter::Peekable<I>,
    tokens: &mut Vec<Token>,
) -> Result<(), LexerError> {
    let mut collected_lexeme = String::new();
    //TODO: Match alphanumeric identifiers
    loop {
        match iter.peek() {
            Some(c) => match c {
                c if c.is_alphabetic() => {
                    collected_lexeme.push(c.to_owned());
                    iter.next();
                }
                _ => {
                    if !collected_lexeme.is_empty() {
                        tokens.push(Token {
                            lexeme: collected_lexeme,
                            metadata: TokenMetadata {
                                char_number: 0,
                                line_number: 0,
                            },
                            token_type: Some(TokenType::IDENTIFIER),
                        });
                    }
                    return Ok(());
                }
            },
            None => return Err(LexerError::UnexpectedEOF), // None => return Err("Unexpected EOF while processing whitespace.".to_string()),
        }
    }
}
