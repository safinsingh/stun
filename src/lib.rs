pub struct Lexer<'a> {
    pub input: &'a str,
    pub cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input,
            cursor: 0,
        }
    }

    fn peek(&self, chars: usize) -> Option<&'a str> {
        if self.input.len() <= self.cursor + chars {
            None
        } else {
            Some(&self.input[self.cursor..self.cursor + chars])
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.get_char(self.cursor + 1)
    }

    fn translate(&mut self, chars: usize) {
        self.cursor += chars;
    }

    fn get_char(&self, num: usize) -> Option<char> {
        self.input.chars().nth(num)
    }

    pub fn current_char(&self) -> Option<char> {
        self.get_char(self.cursor)
    }

    pub fn next(&mut self) -> Option<Token> {
        if let Some("--[[") = self.peek(4) {
            None // fix this later
        } else if let Some(c) = self.current_char() {
            let start = self.cursor;
            let next_char = self.peek_char();

            match c {
                '=' if next_char == Some('=') => {
                    self.translate(2);

                    Some(Token::new(
                        TokenKind::Equate,
                        Span::new(start, self.cursor),
                    ))
                }
                '=' => {
                    self.translate(1);

                    Some(Token::new(
                        TokenKind::Assign,
                        Span::new(start, self.cursor),
                    ))
                }
                ' ' => {
                    self.translate(1);
                    self.next()
                }
                _ => {
                    self.translate(1);

                    Some(Token::new(
                        TokenKind::Undefined,
                        Span::new(start, self.cursor),
                    ))
                }
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    fn new(kind: TokenKind, span: Span) -> Self {
        Token {
            kind: kind,
            span: span,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Assign,
    Equate,
    Undefined,
    // Comment(Comment),
    // Integer(i64),
}

#[derive(Debug, PartialEq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    fn new(start: usize, end: usize) -> Self {
        Span {
            start: start,
            end: end,
        }
    }
}

// #[derive(Debug, PartialEq)]
// pub enum Comment {
//     SingleLine(String),
//     MultiLine(String),
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn whitespace() {
        let input = "    ";

        let mut lex = Lexer::new(input);
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn equate_and_assign() {
        let input = "= ==";
        let mut lex = Lexer::new(input);

        assert_eq!(
            lex.next(),
            Some(Token::new(TokenKind::Assign, Span::new(0, 1),))
        );

        assert_eq!(
            lex.next(),
            Some(Token::new(TokenKind::Equate, Span::new(2, 4),))
        );
    }
}
