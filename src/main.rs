use std::borrow::Cow;
use std::env;
use std::fmt;
use std::fs;

struct Equals {}

impl Equals {
    fn new() -> Self {
        Equals {}
    }
}

enum Token {
    Integer(i64),
    Equals(Equals),
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(_) => write!(f, "Integer"),
            Self::Equals(_) => write!(f, "Plus"),
            Self::Eof => write!(f, "Eof"),
        }
    }
}

struct TokOk<'a> {
    literal: Cow<'a, str>,
    parsed: Token,
    tok_type: String,
}

struct TokErr<'a> {
    literal: &'a str,
}

impl<'a> TokOk<'a> {
    fn new(l: Cow<'a, str>, p: Token, t: String) -> Self {
        TokOk {
            literal: l,
            parsed: p,
            tok_type: t,
        }
    }
}

impl<'a> TokErr<'a> {
    fn new(l: &'a str) -> Self {
        TokErr { literal: l }
    }
}

enum TokResult<'a> {
    Ok(TokOk<'a>),
    Err(TokErr<'a>),
}

fn lex<'a>(input: &'a str) -> TokResult<'a> {
    if input.chars().all(char::is_numeric) {
        return TokResult::Ok(TokOk::new(
            Cow::from(input),
            Token::Integer(input.parse::<i64>().unwrap()),
            "Integer".into(),
        ));
    } else if input == "=" {
        return TokResult::Ok(TokOk::new(
            Cow::from(input),
            Token::Equals(Equals::new()),
            "Equals".into(),
        ));
    } else {
        TokResult::Err(TokErr::new(input))
    }
}

impl<'a> fmt::Display for TokResult<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokResult::Ok(o) => write!(f, "TokResult::Ok({}: {})", o.literal, o.tok_type),
            TokResult::Err(e) => write!(f, "TokResult::Err({})", e.literal),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let content = fs::read_to_string(&file)?;
    for line in content.lines() {
        let toks = line.trim();
        for tok in toks.split_whitespace() {
            println!("{}", lex(tok));
        }
    }

    Ok(())
}
