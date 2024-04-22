use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Made by Tanvir
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let word_count = parse_input!(input_line, i32); // Number of words in the word set
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    for word in inputs.split_whitespace() {
    }

    // game loop
    loop {
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();
        for i in inputs.split_whitespace() {
            let state = parse_input!(i, i32);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("ANSWER");
    }
}
