use std::{env, error::Error, fs};

mod lib;
use lib::Lexer;

fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();
	let file = &args[1];

	let content = fs::read_to_string(&file)?;

	for line in content.lines() {
		let mut lex = Lexer::new(line.trim());

		loop {
			println!("{:?}", lex.next());

			if lex.current_char().is_none() {
				break;
			}
		}
	}

	Ok(())
}
