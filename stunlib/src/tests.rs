#[cfg(test)]
mod test {
	use crate::lex::*;

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
			Some(Token::new(TokType::Assign, Span::new(0, 1, 1)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Equate, Span::new(2, 4, 1)))
		);
	}

	#[test]
	fn var_assign_num() {
		let input = "      x=  23  ";
		let mut lex = Lexer::new(input);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Ident("x".into()), Span::new(6, 7, 1)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Assign, Span::new(7, 8, 1)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Number(23.0), Span::new(10, 12, 1)))
		);
	}

	#[test]
	fn var_assign_bool() {
		let input = "xgfd =true";
		let mut lex = Lexer::new(input);

		assert_eq!(
			lex.next(),
			Some(Token::new(
				TokType::Ident("xgfd".into()),
				Span::new(0, 4, 1)
			))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Assign, Span::new(5, 6, 1)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::True, Span::new(6, 10, 1)))
		);
	}

	#[test]
	fn single_line_comment() {
		let input = "-- hello world!";
		let mut lex = Lexer::new(input);

		assert_eq!(
			lex.next(),
			Some(Token::new(
				TokType::SingleLineComment("hello world!".into()),
				Span::new(0, 15, 1)
			))
		);
	}

	#[test]
	fn single_line_string() {
		let input = "x=\"str\"";
		let mut lex = Lexer::new(input);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Ident("x".into()), Span::new(0, 1, 1)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Assign, Span::new(1, 2, 1)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(
				TokType::SingleLineString("str".into()),
				Span::new(2, 8, 1)
			))
		);
	}

	#[test]
	fn var_assign_float() {
		let input = "      x=  2.3  ";
		let mut lex = Lexer::new(input);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Ident("x".into()), Span::new(6, 7, 1)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Assign, Span::new(7, 8, 1)))
		);

		assert_eq!(
			lex.next(),
			Some(Token::new(TokType::Number(2.3), Span::new(10, 13, 1)))
		);
	}
}
