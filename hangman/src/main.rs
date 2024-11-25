use std::io;
use std::collections::HashSet;

fn main() {
    println!("Hangman!");

    let secret = "RUST".to_uppercase();

    let mut guesses = HashSet::new();

    let mut attempts = 6;

    loop {
        let progress: String = secret
            .chars()
            .map(|c| {
                if guesses.contains(&c) { c } else { '_' }
            })
            .collect();
        println!("Word {}", progress);

        if !progress.contains("_") {
            println!("Word guessed correctly {}", secret);
            break;
        }

        println!("Attempts left {}", attempts);

        let input = get_input("Guess a letter").to_uppercase();

        if input.len() != 1 || !input.chars().all(|c| c.is_alphabetic()) {
            println!("Invalid input! Please enter a single letter.");
            continue;
        }

        let letter = input.chars().next().unwrap();
        if guesses.contains(&letter) {
            println!("You already guessed {} earlier", letter);
            continue;
        }

        guesses.insert(letter);

        if secret.contains(letter) {
            println!("Correct! '{}' is in the word.", letter);
        } else {
            attempts -= 1;
            println!("Incorrect! '{}' is not in the word.", letter);
        }

        if attempts == 0 {
            println!("Game over! The word was: {}", secret);
            break;
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Error reading input");
    s.trim().to_string()
}
