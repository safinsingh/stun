use std::iter::Iterator;
mod tests;

impl<'a> Iterator for Lexer<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Token> {
		if let Some(c) = self.current_char() {
			let next_char = self.peek_char();
			match c {
				'\n' => self.newline(),
				'-' if next_char == Some('-') => self.single_line_comment(),
				'"' => self.single_line_string(),
				' ' | '\t' => self.whitespace(),
				'=' if next_char == Some('=') => self.equate(),
				'=' => self.assign(),
				'(' => self.single_char_match(TokType::LParen),
				')' => self.single_char_match(TokType::RParen),
				'{' => self.single_char_match(TokType::LBrace),
				'}' => self.single_char_match(TokType::RBrace),
				':' => self.single_char_match(TokType::Colon),
				'.' => self.single_char_match(TokType::Period),
				'>' if next_char == Some('=') => {
					self.multi_char_match(2, TokType::GreaterEq)
				}

				'<' if next_char == Some('=') => {
					self.multi_char_match(2, TokType::LessEq)
				}
				'>' => self.single_char_match(TokType::Greater),
				'<' => self.single_char_match(TokType::Less),
				'+' => self.single_char_match(TokType::Plus),
				'-' => self.single_char_match(TokType::Minus),
				'*' => self.single_char_match(TokType::Multiply),
				'/' => self.single_char_match(TokType::Divide),
				'%' => self.single_char_match(TokType::Mod),
				'~' if next_char == Some('=') => {
					self.multi_char_match(2, TokType::NoEq)
				}
				'A'..='Z' | 'a'..='z' | '_' => self.identifier(),
				'0'..='9' => self.get_number(),
				_ => self.undefined(),
			}
		} else {
			None
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum TokType {
	// Operators
	Assign,
	Equate,
	Plus,
	Minus,
	Multiply,
	Divide,
	Mod,
	NoEq,

	// Keywords
	True,
	False,
	If,
	Function,
	End,
	Local,
	Nil,
	While,
	Then,
	And,
	Or,
	Not,
	For,
	Do,

	// Symbols
	LParen,
	RParen,
	LBrace,
	RBrace,
	Colon,
	Period,
	LessEq,
	GreaterEq,
	Greater,
	Less,

	// Internals
	Newline,
	Undefined(String),

	// Primitives
	Number(f64),
	SingleLineString(String),

	// Comments
	SingleLineComment(String),

	// Misc
	Ident(String),
}

pub struct Lexer<'a> {
	pub(crate) input: &'a str,
	pub(crate) cursor: usize,
	pub(crate) pseudo_cursor: usize,
	pub(crate) line: usize,
}

impl<'a> Lexer<'a> {
	// parsing methods
	pub(crate) fn single_char_match(&mut self, t: TokType) -> Option<Token> {
		self.translate(1);
		Some(Token::new(
			t,
			Span::new(self.pseudo_cursor - 1, self.pseudo_cursor, self.line),
		))
	}

	pub(crate) fn multi_char_match(
		&mut self,
		chars: usize,
		t: TokType,
	) -> Option<Token> {
		self.translate(chars);
		Some(Token::new(
			t,
			Span::new(self.pseudo_cursor - 1, self.pseudo_cursor, self.line),
		))
	}

	pub(crate) fn get_number(&mut self) -> Option<Token> {
		let mut s = String::new();
		let start = self.pseudo_cursor;

		while let Some(n) = self.current_char() {
			match n {
				'0'..='9' => {
					s.push(n);
					self.translate(1);
				}
				'.' if matches!(self.peek_char(), Some('0'..='9')) => {
					s.push(n);
					self.translate(1);
				}
				_ => break,
			}
		}

		match s.parse() {
			Ok(num) => Some(Token {
				kind: TokType::Number(num),
				span: Span::new(start, self.pseudo_cursor, self.line),
			}),
			_ => Some(Token {
				kind: TokType::Undefined(s),
				span: Span::new(start, self.cursor, self.line),
			}),
		}
	}

	pub(crate) fn single_line_string(&mut self) -> Option<Token> {
		let mut s = String::new();
		let start = self.pseudo_cursor;

		self.translate(1);
		while let Some(ch) = self.current_char() {
			match ch {
				'\\' if self.peek_char() == Some('"') => {
					s.push('"');
					self.translate(2);
				}
				'"' => {
					self.translate(2);
					break;
				}
				_ => {
					s.push(ch);
					self.translate(1);
				}
			}
		}

		Some(Token::new(
			TokType::SingleLineString(s),
			Span::new(start, self.pseudo_cursor, self.line),
		))
	}

	pub(crate) fn identifier(&mut self) -> Option<Token> {
		let mut s = String::new();
		let start = self.pseudo_cursor;

		while let Some(ch) = self.current_char() {
			match ch {
				'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {
					s.push(ch);
					self.translate(1);
				}
				_ => break,
			}
		}

		let kind = match s.as_str() {
			"true" => TokType::True,
			"false" => TokType::False,
			"if" => TokType::If,
			"function" => TokType::Function,
			"end" => TokType::End,
			"local" => TokType::Local,
			"nil" => TokType::Nil,
			"while" => TokType::While,
			"then" => TokType::Then,
			"and" => TokType::And,
			"or" => TokType::Or,
			"not" => TokType::Not,
			"for" => TokType::For,
			"do" => TokType::Do,
			_ => TokType::Ident(s),
		};

		Some(Token::new(
			kind,
			Span::new(start, self.pseudo_cursor, self.line),
		))
	}

	pub(crate) fn single_line_comment(&mut self) -> Option<Token> {
		let mut s = String::new();
		let start = self.pseudo_cursor;

		self.translate(3);
		while let Some(ch) = self.current_char() {
			match ch {
				'\n' => break,
				_ => {
					s.push(ch);
					self.translate(1);
				}
			}
		}

		Some(Token::new(
			TokType::SingleLineComment(s),
			Span::new(start, self.pseudo_cursor, self.line),
		))
	}

	pub(crate) fn newline(&mut self) -> Option<Token> {
		self.translate(1);
		self.pseudo_cursor = 0;
		self.line += 1;

		Some(Token::new(
			TokType::Newline,
			Span::new(0, self.pseudo_cursor, self.line),
		))
	}

	pub(crate) fn equate(&mut self) -> Option<Token> {
		let start = self.pseudo_cursor;
		self.translate(2);

		Some(Token::new(
			TokType::Equate,
			Span::new(start, self.pseudo_cursor, self.line),
		))
	}

	pub(crate) fn assign(&mut self) -> Option<Token> {
		let start = self.pseudo_cursor;
		self.translate(1);

		Some(Token::new(
			TokType::Assign,
			Span::new(start, self.pseudo_cursor, self.line),
		))
	}

	pub(crate) fn whitespace(&mut self) -> Option<Token> {
		self.translate(1);
		self.next()
	}

	pub(crate) fn undefined(&mut self) -> Option<Token> {
		let start = self.pseudo_cursor;
		self.translate(1);

		Some(Token::new(
			TokType::Undefined(self.current_char().unwrap().into()),
			Span::new(start, self.pseudo_cursor, self.line),
		))
	}

	// utils
	pub fn new(input: &'a str) -> Self {
		Lexer {
			input,
			cursor: 0,
			pseudo_cursor: 0,
			line: 1,
		}
	}

	pub(crate) fn peek_char(&self) -> Option<char> {
		self.get_char(self.cursor + 1)
	}

	pub(crate) fn translate(&mut self, chars: usize) {
		self.cursor += chars;
		self.pseudo_cursor += chars;
	}

	pub(crate) fn get_char(&self, num: usize) -> Option<char> {
		self.input.chars().nth(num)
	}

	pub(crate) fn current_char(&self) -> Option<char> {
		self.get_char(self.cursor)
	}
}

#[derive(Debug, PartialEq)]
pub struct Token {
	pub kind: TokType,
	pub span: Span,
}

impl Token {
	pub(crate) fn new(kind: TokType, span: Span) -> Self {
		Token { kind, span }
	}
}

#[derive(Debug, PartialEq)]
pub struct Span {
	start: usize,
	end: usize,
	line: usize,
}

impl Span {
	pub(crate) fn new(start: usize, end: usize, line: usize) -> Self {
		Span { start, end, line }
	}
}
