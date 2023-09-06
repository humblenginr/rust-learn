use lexer::LexemeGenerator;

pub mod lexer;
pub mod translator;

fn main() {
    let input = String::from("8+9-3+4+9+8+1-2-2-3");
    // let mut translator = Translator::new(&input);
    // if let Err(e) = translator.translate() {
    //     print!("{}", e);
    // }
    let mut lexer = LexemeGenerator::new(&input);
    if let Some(token) = lexer.scan() {
        println!("{}", token);
    }
    if let Some(token) = lexer.scan() {
        println!("{}", token);
    }

    if let Some(token) = lexer.scan() {
        println!("{}", token);
    }
    if let Some(token) = lexer.scan() {
        println!("{}", token);
    }
    if let Some(token) = lexer.scan() {
        println!("{}", token);
    }

    if let Some(token) = lexer.scan() {
        println!("{}", token);
    }
}
