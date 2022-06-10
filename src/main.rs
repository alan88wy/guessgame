use std::io::{self, Write};

fn main() {
    println!("Guess the number!");

    loop {
        let mut guess = String::new();

        print!("Please input your guess : ");

        io::stdout().flush().expect("Cannot flush stdout");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // if guess.trim().to_lowercase() == "q" {

        if guess.trim().to_lowercase().eq("q") {
            println!("See ya!!");
            break;
        } else {
            // match user_input
            let guess: i64 = match guess.trim().parse::<i64>() {
                Ok(inp) => inp,
                Err(_) => {
                    println!("Error, please enter a number!");
                    continue;
                }
            };
            println!("You guessed: {}", guess);
        }
    }
}
