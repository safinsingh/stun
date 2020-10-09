use std::{env, error::Error, fs};

use stunlex::Lexer;

fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();
	let file = &args[1];

	let content = fs::read_to_string(&file)?;
	let lex = Lexer::new(&content);

	for lexed in lex {
		println!("{:?}", lexed)
	}

	Ok(())
}
