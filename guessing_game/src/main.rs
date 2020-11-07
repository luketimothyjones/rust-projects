use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guess the number\n");    
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        print!("Guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed '{}'", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small\n"),
            Ordering::Greater => println!("Too large\n"),
            Ordering::Equal => {
                println!("\nCorrect! You win!\n");
                break;
            }
        }
    }
}
