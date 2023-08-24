// START -> DECLARATION | ASSIGNMENT;
// DECLARATION -> EVENT
// ASSIGNMENT -> const IDENTIFIER =
// IDENTIFIER

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

#[derive(Debug)]
pub enum LexerError {
    IO(std::io::ErrorKind),
    UnexpectedEOF,
    UnexpectedDeclaration,
}
impl From<std::io::Error> for LexerError {
    fn from(error: std::io::Error) -> Self {
        LexerError::IO(error.kind())
    }
}
/// Lexical analyzer. Takes tokenized input and generates an abstract syntax tree.
pub fn scan<I: Iterator<Item = char>>(
    iter: &mut std::iter::Peekable<I>,
) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    let result = start(iter, &mut tokens);
    match result {
        Ok(_) => {
            return Ok(tokens);
        }
        Err(err) => {
            return Err(err);
        }
    }
}

fn expect<I: Iterator<Item = char>>(
    iter: &mut std::iter::Peekable<I>,
    lexeme: &str,
    token_type: Option<TokenType>,
) -> Result<Token, LexerError> {
    for x in lexeme.chars() {
        if iter.next().unwrap_or_default() != x {
            // return Err("Collection Error".to_string());
            return Err(LexerError::UnexpectedEOF);
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

fn start<I: Iterator<Item = char>>(
    iter: &mut std::iter::Peekable<I>,
    tokens: &mut Vec<Token>,
) -> Result<(), LexerError> {
    loop {
        match iter.peek() {
            Some(c) => match c {
                w if w.is_whitespace() => {
                    whitespace(iter)?;
                    return Ok(());
                }
                _ => {
                    declaration(iter, tokens)?;
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
                _ => {
                    return Ok(());
                }
            },
            None => return Ok(()),
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
