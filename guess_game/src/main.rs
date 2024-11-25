use rand::Rng;
use std::io;

fn main() {
    println!("Guessing game");

    let random = rand::thread_rng().gen_range(1..=100);

    let mut attempts = 0;


    loop {
        if attempts < 3 {
            let input = get_input("Guess a number: ").trim().parse::<i32>();

            match input {
                Ok(guess) => {
                    if guess == random {
                        println!("Your guess is correct");
                        break;
                    } else if guess < random {
                        println!("Your guess is lower");
                    } else {
                        println!("Your guess is higher")
                    }
                }
                Err(_) => {
                    println!("Invalid input");
                }
            }

            attempts += 1;
        } else {
            println!("You lost the correct number was {}", random);
            break;
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Error reading input");
    s
}