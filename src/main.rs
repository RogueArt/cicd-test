mod math;

// For generating our guess number
use rand::{thread_rng, Rng};

use std::ascii::AsciiExt;
// For getting input from the console
use std::io::{self, stdin, Write};
use std::process::{self, Command, Stdio};

const MIN: u8 = 0;
const MAX: u8 = 50;

fn main() {
    loop {
        // Generate the target number ahead of time 
        let mut rng = thread_rng();
        let target_num: u8 = rng.gen_range(MIN..MAX);

        // Begin the guessing game
        println!("Let's play a guessing game!");
        println!("Guess a number between {} and {}.", MIN, MAX);
        loop {
            print!("Guess: ");
            io::stdout().flush().expect("Something went wrong flushing console output.");

            // Get the guess
            let mut input = String::new();
            stdin().read_line(&mut input).expect("You entered an invalid input.");

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
                    println!("Try again with a number from {} to {}", MIN, MAX);
                    continue;
                }
            };    
        }

        // If broke out of inner loop, means you win the game
        println!("Congratulations! You win! The number was {}", target_num);

        // Ask if player wants to play again, check their response
        println!("Would you like to play again? (Y/N)");
        let mut response = String::new();
        stdin().read_line(&mut response).expect("You entered an invalid input.");

        // Repeat the game if starts with y, otherwise assume break
        if response.trim().to_ascii_lowercase().starts_with('y') { continue; }
        else { break; }
    }
}
