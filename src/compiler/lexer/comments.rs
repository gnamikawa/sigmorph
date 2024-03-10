use crate::compiler::LexerError;
use rstest::rstest;

use crate::compiler::lexer::expect;

pub fn comment<I: Iterator<Item = char>>(
    iter: &mut std::iter::Peekable<I>,
) -> Result<(), LexerError> {
    expect(iter, "//", None)?;
    loop {
        match iter.peek() {
            None | Some('\n') => return Ok(()),
            Some(_) => {
                iter.next();
                return Ok(());
            }
        }
    }
}

#[rstest]
#[case::comment_start("//")]
#[case::alphabet_lowercase("//abcdefghijklmnopqrstuvwxyz")]
#[case::alphabet_uppercase("//ABCDEFGHIJKLMNOPQRSTUVWXYZ")]
#[case::numeric("//0123456789")]
#[case::single_space("// ")]
#[case::linebreak_after_comment("//\n")]
#[case::japanese_hiragana("//ひらがな")]
#[case::japanese_kanji("//漢字")]
#[should_panic]
#[case::only_one_slash("/")]
#[should_panic]
#[case::not_a_comment("a")]
#[should_panic]
#[case::empty_string("")]
fn scanner_fn(#[case] lexeme: &str) {
    let mut peekable = lexeme.chars().peekable();
    let result = comment(&mut peekable);
    assert!(!result.is_err())
}
