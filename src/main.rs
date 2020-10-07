use std::env;
use std::fs;

mod lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let content = fs::read_to_string(&file)?;
    for line in content.lines() {
        let mut lex = lib::Lexer::new(line.trim());

        loop {
            println!("{:?}", lex.next());

            if lex.current_char().is_none() {
                break;
            }
        }
    }

    Ok(())
}
