mod math;

// For generating our guess number
use rand::{thread_rng, Rng};

// For getting input from the console
use std::io::{self, stdin, Write};
use std::process::{self, Command, Stdio};

fn main() {
    // Generate the target number ahead of time 
    let mut rng = thread_rng();
    let target_num: u8 = rng.gen_range(0..50);

    // Begin the guessing game
    println!("Let's play a guessing game!");
    println!("Guess a number between {:?}:", 0..50);
    loop {
        print!("Guess: ");
        io::stdout().flush().expect("Something went wrong flushing console output.");

        // Get the guess
        let mut input = String::new();
        stdin().read_line(&mut input).expect("You entered an invalid string.");

        // If starts with q, assume that they're trying to leave the game
        if input.trim().starts_with('q') { 
            println!("Exiting the game...");
            process::exit(0);
        }

        // Convert it to an int, check if it's the same as our target
        match input.trim().parse::<u8>() {
            Ok(num) => {
                match num {
                    n if n < target_num => println!("{} is too small", n),
                    n if n > target_num => println!("{} is too big", n),
                    _ => break
                }
            },
            // If error, ask them to type it in again
            Err(e) => {
                println!("Try again with a number from {:?}", 0..50);
                continue;
            }
        };    
    }

    // If broke out of statement, means you win the game
    println!("Congratulations! You win! The number was {}", target_num);
}
