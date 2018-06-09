
use std::iter;


#[derive(Debug)]
struct Position {
    file: &'static str,
    line: u32,
    pos: u32
}

impl Position {
    pub fn next(&mut self){
        self.pos = self.pos + 1;
    }
}

pub struct Lexeme<'a> {
    input: iter::Peekable<str::Chars<'a>>,
    pos: Position
}

impl<'a> Lexeme<'a> {

    pub fn new(input: &'a str input, file: &'static str, line: u32){
        let s = input.chars().peekable();
        return Lexime { input: input,
                        pos: Position { file: file,
                                        line: line,
                                        pos: pos };
        }
    }

    pub fn peek(&self) -> Option<char>{
        return self.input.peek().map(|c| *c);
    }
}

impl<'a> Iterator for Lexime<'a>{
    type = Char;
    fn next(&mut self) -> Option<Char>{
        match self.input.next() {
            Some(c) => {
                self.pos.next();
                return Some(c);
            }
            None => None;
        }
    }
}
