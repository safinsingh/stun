#[cfg(test)]
mod test {
	use stun::*;

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
