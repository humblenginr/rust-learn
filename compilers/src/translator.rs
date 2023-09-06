use std::io::Error;
// A Syntax directed translator that converts Infix Arithmetic expressions into their postfix
// notation.
//
// Context-Free-Grammar:
// expr -> expr + term
//         | expr - term
//         | term
// term -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
//
//
// The translation scheme for this grammar is:
// expr -> expr + term {print("+")}
//         | expr - term {print("-")}
//         | term
// term -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 {print(digit)}

// This scheme is left-recursive, so we convert it into right-recursive so that we can use
// predective recursive descent parsing.
//
// A -> Ax | Az | y
//
// A -> yR
// R -> xR | zR |  ''
//
// A = expr
// x = + term {print("+")}
// z = - term {print("-")}
// y = term
//
//
// expr -> term rest
// rest -> + term {print("+")} rest
//         | - term {print("-")} rest
//         | ''
// term -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 {print(digit)}

pub struct Translator {
    input_symobls: Vec<char>,
    lookahead_index: usize,
}

impl Translator {
    pub fn new(input: &str) -> Self {
        Translator {
            input_symobls: input.chars().collect(),
            lookahead_index: 0,
        }
    }

    pub fn translate(&mut self) -> Result<(), Error> {
        self.expr()
    }

    fn get_lookahead(&self) -> char {
        self.input_symobls[self.lookahead_index]
    }

    fn expr(&mut self) -> Result<(), Error> {
        if let Err(e) = self.term() {
            return Err(e);
        }
        if let Err(e) = self.rest() {
            return Err(e);
        }

        Ok(())
    }
    fn match_lookahead(&mut self, symbol: char) {
        if symbol == self.get_lookahead() {
            self.lookahead_index += 1;
        }
    }
    fn rest(&mut self) -> Result<(), Error> {
        match self.get_lookahead() {
            '+' => {
                self.match_lookahead('+');
                if let Err(e) = self.term() {
                    return Err(e);
                }
                print!("+");
                if let Err(e) = self.rest() {
                    return Err(e);
                }
                return Err(Error::new(std::io::ErrorKind::Other, "Syntax Error"));
            }
            '-' => {
                self.match_lookahead('-');
                if let Err(e) = self.term() {
                    return Err(e);
                }
                print!("-");
                if let Err(e) = self.rest() {
                    return Err(e);
                }
                return Err(Error::new(std::io::ErrorKind::Other, "Syntax Error"));
            }
            _ => {
                return Err(Error::new(std::io::ErrorKind::Other, "Syntax Error"));
            }
        }
    }
    fn term(&mut self) -> Result<(), Error> {
        match self.get_lookahead() {
            '0' => {
                print!("0");
                self.match_lookahead('0');
                Ok(())
            }
            '1' => {
                print!("1");
                self.match_lookahead('1');
                Ok(())
            }
            '2' => {
                print!("2");
                self.match_lookahead('2');
                return Ok(());
            }
            '3' => {
                print!("3");
                self.match_lookahead('3');
                return Ok(());
            }
            '4' => {
                print!("4");
                self.match_lookahead('4');
                return Ok(());
            }
            '5' => {
                print!("5");
                self.match_lookahead('5');
                return Ok(());
            }
            '6' => {
                print!("6");
                self.match_lookahead('6');
                return Ok(());
            }
            '7' => {
                print!("7");
                self.match_lookahead('7');
                return Ok(());
            }
            '8' => {
                print!("8");
                self.match_lookahead('8');
                return Ok(());
            }
            '9' => {
                print!("9");
                self.match_lookahead('9');
                return Ok(());
            }
            _ => Err(Error::new(std::io::ErrorKind::Other, "Syntax Error")),
        }
    }
}
