use std::{env, fmt::Debug, fs::read_to_string};

use anyhow::anyhow;
use thiserror::Error;

fn main() -> anyhow::Result<()> {
    let path = env::args().nth(1).ok_or_else(|| anyhow!("No input file"))?;
    let input = read_to_string(&path)?;

    let parsed = parse_lines(&input);

    println!(
        "The answer to the first part is {}",
        score_corrupted(&parsed)
    );
    println!(
        "The answer to the second part is {}",
        score_incomplete(&parsed)
    );

    Ok(())
}

fn parse_lines(input: &str) -> Vec<Result<Syntax, ParseError>> {
    input.trim().split('\n').map(Syntax::parse).collect()
}

fn score_corrupted(parsed: &[Result<Syntax, ParseError>]) -> usize {
    parsed
        .iter()
        .filter_map(|r| match r {
            Err(ParseError::UnmatchedToken {
                token: Token { chr, .. },
                ..
            }) => Some(match chr {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            }),
            _ => None,
        })
        .sum()
}

fn score_incomplete(parsed: &[Result<Syntax, ParseError>]) -> usize {
    let mut scores = parsed
        .iter()
        .filter_map(|r| match r {
            Ok(Syntax { completions, .. }) => {
                let score = completions
                    .chars()
                    .map(|c| match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    })
                    .fold(0, |a, v| a * 5 + v);
                Some(score)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[derive(Clone, Copy, PartialEq)]
pub struct Token {
    chr: char,
    typ: TokenType,
    cat: TokenCategory,
}

impl Token {
    fn parse(chr: char) -> Option<Self> {
        match chr {
            '(' => Some(Token {
                chr,
                typ: TokenType::Left,
                cat: TokenCategory::Parenthesis,
            }),
            ')' => Some(Token {
                chr,
                typ: TokenType::Right,
                cat: TokenCategory::Parenthesis,
            }),
            '[' => Some(Token {
                chr,
                typ: TokenType::Left,
                cat: TokenCategory::SquareBracket,
            }),
            ']' => Some(Token {
                chr,
                typ: TokenType::Right,
                cat: TokenCategory::SquareBracket,
            }),
            '{' => Some(Token {
                chr,
                typ: TokenType::Left,
                cat: TokenCategory::CurlyBracket,
            }),
            '}' => Some(Token {
                chr,
                typ: TokenType::Right,
                cat: TokenCategory::CurlyBracket,
            }),
            '<' => Some(Token {
                chr,
                typ: TokenType::Left,
                cat: TokenCategory::AngleBracket,
            }),
            '>' => Some(Token {
                chr,
                typ: TokenType::Right,
                cat: TokenCategory::AngleBracket,
            }),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum TokenType {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
enum TokenCategory {
    Parenthesis,
    SquareBracket,
    CurlyBracket,
    AngleBracket,
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chr)
    }
}

#[derive(Debug)]
struct Syntax {
    tokens: Vec<Token>,
    completions: String,
}

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Invalid character {char:?} at {position:?}")]
    InvalidCharacter { char: char, position: usize },

    #[error("Token {token:?} at {position:?} doesn't match with previous token {prev:?}")]
    UnmatchedToken {
        token: Token,
        prev: Option<Token>,
        position: usize,
    },
}

impl Syntax {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let tokens = input
            .chars()
            .enumerate()
            .map(|(position, c)| {
                Token::parse(c).ok_or(ParseError::InvalidCharacter { char: c, position })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut stack = Vec::new();

        for (p, t) in tokens.iter().enumerate() {
            match t.typ {
                TokenType::Left => {
                    stack.push(t);
                    Ok(())
                }
                TokenType::Right => {
                    let prev = stack.pop().ok_or_else(|| ParseError::UnmatchedToken {
                        token: *t,
                        prev: None,
                        position: p,
                    })?;

                    if prev.cat != t.cat && prev.typ == TokenType::Left {
                        Err(ParseError::UnmatchedToken {
                            token: *t,
                            prev: Some(*prev),
                            position: p,
                        })
                    } else {
                        Ok(())
                    }
                }
            }?;
        }

        let completions = stack
            .into_iter()
            .rev()
            .map(|t| match t.chr {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => unreachable!(),
            })
            .collect();

        Ok(Self {
            tokens,
            completions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_syntax_parsing() {
        assert!(Syntax::parse("()[]{}<>").is_ok());
        assert!(Syntax::parse("([<>]){}").is_ok());
        assert_eq!(
            Syntax::parse("()foo").unwrap_err(),
            ParseError::InvalidCharacter {
                char: 'f',
                position: 2
            }
        );
        assert_eq!(
            Syntax::parse("]").unwrap_err(),
            ParseError::UnmatchedToken {
                token: Token::parse(']').unwrap(),
                prev: None,
                position: 0
            }
        );
        assert_eq!(
            Syntax::parse("(]").unwrap_err(),
            ParseError::UnmatchedToken {
                token: Token::parse(']').unwrap(),
                prev: Some(Token::parse('(').unwrap()),
                position: 1
            }
        );
        assert_eq!(
            Syntax::parse("(([]<)>").unwrap_err(),
            ParseError::UnmatchedToken {
                token: Token::parse(')').unwrap(),
                prev: Some(Token::parse('<').unwrap()),
                position: 5
            }
        );
    }

    #[test]
    fn test_completions() {
        let parsed = Syntax::parse("[({(<(())[]>[[{[]{<()<>>").unwrap();
        assert_eq!(parsed.completions, "}}]])})]");
    }

    #[test]
    fn test_scoring() {
        let parsed = parse_lines(indoc! {"
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]
        "});

        assert_eq!(score_corrupted(&parsed), 26397);
        assert_eq!(score_incomplete(&parsed), 288957);
    }
}
