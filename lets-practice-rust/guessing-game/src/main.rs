use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*; 

fn main() {
    println!("Guess the number in between (1-100)!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    
    // Remove this line in final version - it's for debugging
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;  // This skips the rest of the loop and asks for input again
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small! Try a larger number.".red()),
            Ordering::Greater => println!("{}", "Too big! Try a smaller number.".red()),
            Ordering::Equal => {
                println!("{}", "Congratulations, You win!".green());
                break;  // This exits the loop when player wins
            }
        }
    }
}

// Hint: Find answer using Binary Search 
