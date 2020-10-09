use std::{io, io::Write};

use stunlex::Lexer;

pub(crate) fn new() {
	println!("Stun v0.1.0 REPL");
	println!("Type `exit` to exit.");

	loop {
		print!("> ");
		if let Err(e) = io::stdout().flush() {
			panic!("Fatal error: failed to flush stdout: {}", e)
		}

		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("error: unable to read user input");

		match input.trim() {
			"exit" => break,
			_ => {
				let lex = Lexer::new(input.trim());
				for lexed in lex {
					println!("{:?}", lexed)
				}
			}
		}
	}
}
