use std::{env, error::Error, fs};

use stunlex::Lexer;
mod repl;

fn main() -> Result<(), Box<dyn Error>> {
	let mut args: Vec<String> = env::args().collect();

	match args.len() {
		1 => repl::new(),
		_ => {
			args.remove(0);

			for file in args {
				let content = fs::read_to_string(&file)?;
				let lex = Lexer::new(&content);

				for lexed in lex {
					println!("{:?}", lexed)
				}
			}
		}
	}

	Ok(())
}
