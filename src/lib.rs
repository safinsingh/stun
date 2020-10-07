use std::iter::Iterator;

pub struct Lexer<'a> {
	pub input: &'a str,
	pub cursor: usize,
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Self {
		Lexer { input, cursor: 0 }
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

	fn get_number(&mut self) -> Option<Token> {
		let mut s = String::new();
		let start = self.cursor;

		while let Some(n) = self.current_char() {
			match n {
				'0'..='9' => {
					s.push(n);
					self.translate(1);
				}
				_ => break,
			}
		}

		match s.parse() {
			Ok(num) => Some(Token {
				kind: TokType::Number(num),
				span: Span::new(start, self.cursor),
			}),
			_ => Some(Token {
				kind: TokType::Undefined(s.clone()),
				span: Span::new(start, self.cursor),
			}),
		}
	}

	fn identifier(&mut self) -> Option<Token> {
		let mut s = String::new();
		let start = self.cursor;

		while let Some(n) = self.current_char() {
			match n {
				'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {
					s.push(n);
					self.translate(1);
				}
				_ => break,
			}
		}

		let kind = match s.as_str() {
			"true" => TokType::True,
			"false" => TokType::False,
			_ => TokType::Ident(s),
		};

		Some(Token::new(kind, Span::new(start, self.cursor)))
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Token> {
		if let Some("--") = self.peek(2) {
			self.translate(self.input.len() - 1);

			Some(Token::new(
				TokType::SingleLineComment(self.input[2..].into()),
				Span::new(0, self.cursor),
			))
		} else if let Some(c) = self.current_char() {
			let start = self.cursor;
			let next_char = self.peek_char();

			match c {
				'=' if next_char == Some('=') => {
					self.translate(2);

					Some(Token::new(
						TokType::Equate,
						Span::new(start, self.cursor),
					))
				}
				'=' => {
					self.translate(1);

					Some(Token::new(
						TokType::Assign,
						Span::new(start, self.cursor),
					))
				}
				' ' => {
					self.translate(1);
					self.next()
				}
				'A'..='Z' | 'a'..='z' | '_' => self.identifier(),
				'0'..='9' => self.get_number(),
				_ => {
					self.translate(1);

					Some(Token::new(
						TokType::Undefined(c.into()),
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
	pub kind: TokType,
	pub span: Span,
}

impl Token {
	fn new(kind: TokType, span: Span) -> Self {
		Token { kind, span }
	}
}

#[derive(Debug, PartialEq)]
pub enum TokType {
	Assign,
	Equate,
	Undefined(String),
	Ident(String),
	True,
	False,
	SingleLineComment(String),
	Number(f64),
}

#[derive(Debug, PartialEq)]
pub struct Span {
	start: usize,
	end: usize,
}

impl Span {
	fn new(start: usize, end: usize) -> Self {
		Span { start, end }
	}
}

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
			Some(Token::new(TokType::Assign, Span::new(0, 1)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Equate, Span::new(2, 4)))
		);
	}

	#[test]
	fn var_assign_num() {
		let input = "      x=  23  ";
		let mut lex = Lexer::new(input);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Ident("x".into()), Span::new(6, 7)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Assign, Span::new(7, 8)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Number(23.0), Span::new(10, 12)))
		);
	}

	#[test]
	fn var_assign_bool() {
		let input = "xgfd =true";
		let mut lex = Lexer::new(input);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Ident("xgfd".into()), Span::new(0, 4)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Assign, Span::new(5, 6)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::True, Span::new(6, 10)))
		);
	}

	#[test]
	fn single_line_comment() {
		let input = "--whats up, cool comment, huh?";
		let mut lex = Lexer::new(input);

		assert_eq!(
			lex.next(),
			Some(Token::new(
				TokType::SingleLineComment(
					"whats up, cool comment, huh?".into()
				),
				Span::new(0, 29)
			))
		);
	}
}
