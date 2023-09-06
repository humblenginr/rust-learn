use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

#[derive(Clone)]
pub enum Token {
    Num(usize),
    Id(String),
    Add,
    Subract,
    Divide,
    Multiply,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Num(n) => write!(f, "Num({})", n),
            Self::Id(i) => write!(f, "Id({})", i),
            Self::Add => write!(f, "Add(+)"),
            Self::Subract => write!(f, "Subract(-)"),
            Self::Divide => write!(f, "Divide(/)"),
            Self::Multiply => write!(f, "Multiply(*)"),
        }
    }
}

// A lexeme generator will take in stream of strings as it's input and return Tokens as it's output
// Given a Vec<char> as its input, for every call to `scan`, it will return a Token
pub struct LexemeGenerator {
    symbol_table: HashMap<String, Token>,
    source_code: Vec<char>,
    current_index: usize,
}

impl LexemeGenerator {
    pub fn new(input: &str) -> Self {
        let mut hsh_map = HashMap::new();
        hsh_map.insert(String::from("+"), Token::Add);
        hsh_map.insert(String::from("-"), Token::Subract);
        hsh_map.insert(String::from("*"), Token::Multiply);
        hsh_map.insert(String::from("/"), Token::Divide);

        LexemeGenerator {
            symbol_table: hsh_map,
            source_code: input.chars().collect(),
            current_index: 0,
        }
    }

    // This function scan's the source_code and tries to make a Token out of it
    // Check if the single digit makes up any token
    // if it does not, check if it makes up a Number
    //
    // Number, Identifier
    // An Identifier cannot start with a Numeric Digit
    // No keyword starts with a Number
    //
    // If the first character is a digit, then that is bound to be a Number
    // Look at the next character, as long as the peek is not a numeric digit, keep on pushing this
    // into the buffer. And then return a Number token
    //
    // If it is not a single character token or a Number, then it is bound to be either a Keyword
    // or an Identifier.
    //
    // Now, keep adding things into the buffer, in every loop, check if the existing contents in
    // the buffer makes up a Keyword, and check if the peek character makes up a Keyword(only
    // keyword). If the peek character makes up a Keyword, then return the existing contents of the
    // buffer as the Identifier token. If the existing contents make up a Keyword, then just return
    // that keyword.
    //
    // let a = 75;
    //      ^
    pub fn scan(&mut self) -> Option<Token> {
        let mut buffer: Vec<char> = vec![];
        loop {
            let current_char = self.source_code[self.current_index];
            self.current_index += 1;

            if let Some(token) = self.symbol_table.get(&current_char.to_string()) {
                if !buffer.is_empty() {
                    self.current_index -= 1;
                    let buffer_contents: String = buffer.iter().collect();
                    if is_string_numeric(&buffer_contents) {
                        let num: usize = buffer_contents.parse().unwrap();
                        return Some(Token::Num(num));
                    }
                    if let Some(token) = self.symbol_table.get(&buffer_contents) {
                        return Some(token.clone());
                    } else {
                        let id_token = Token::Id(buffer_contents.clone());
                        self.symbol_table.insert(buffer_contents, id_token.clone());
                        return Some(id_token);
                    }
                }

                return Some(token.clone());
            } else if current_char == ' ' {
                let buffer_contents: String = buffer.iter().collect();
                if is_string_numeric(&buffer_contents) {
                    let num: usize = buffer_contents.parse().unwrap();
                    return Some(Token::Num(num));
                }
                if let Some(token) = self.symbol_table.get(&buffer_contents) {
                    return Some(token.clone());
                } else {
                    let id_token = Token::Id(buffer_contents.clone());
                    self.symbol_table.insert(buffer_contents, id_token.clone());
                    return Some(id_token);
                }
            } else {
                buffer.push(current_char);
            }
        }
    }
}

fn is_string_numeric(str: &str) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
