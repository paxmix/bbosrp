use std::io::{self, Write};

use rand::seq::SliceRandom;

fn main() {
    const NUM_DIGITS: i8 = 3;
    const MAX_GUESSES: i8 = 10;

    // Initalize the game
    println!(
        "Bagels, a deductive logic game.

I am thinking of a {}-digit number. Try to guess what it is.
Here are some clues:
When I say:         That means:
    Pico            One digit is correct but in the wrong position.
    Fermi           One digit is correct and in the right position.
    Bagels          No digit is correct.",
        NUM_DIGITS
    );

    loop {
        let rand_num = get_num();
        println!("\nI have thought up a number.");
        println!("You have {} guesses to get it.", MAX_GUESSES);

        let mut guess_no = 1;
        while guess_no <= MAX_GUESSES {
            let mut guess = String::new();
            loop {
                guess.clear();
                println!("Guess #{}", &guess_no);
                print!("> ");
                // flush to force the buffer to display "> " immediately
                io::stdout().flush().unwrap();

                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");

                // validate input: 3 length and is decimal
                if guess.trim().len() == 3 && guess.trim().parse::<i32>().is_ok() {
                    break;
                }
            }
            let guess = guess.trim();
            // Right guess, end game
            if guess == rand_num {
                println!("You got it!");
                break;
            }
            // Guesses ran out, exit game
            if guess_no == MAX_GUESSES {
                println!("You lost!");
                println!("The correct number is {}", rand_num);
                break;
            }
            givehint(guess, &rand_num);
            println!();
            guess_no += 1;
        }

        println!("Do you want to play again? (yes or no)");
        let mut replay = String::new();
        io::stdin()
            .read_line(&mut replay)
            .expect("Failed to read line");
        // Looping the game (Restart) if player enter anything word beginning with the letter "y"
        if !replay.trim().starts_with("y") {
            break;
        }
    }
}

fn givehint(guess: &str, rand_num: &str) {
    let mut clues: Vec<&str> = Vec::new();
    let guess_chars: Vec<char> = guess.chars().collect();
    let rand_num_chars: Vec<char> = rand_num.chars().collect();

    for i in 0..guess_chars.len() {
        if guess_chars[i] == rand_num_chars[i] {
            clues.push("Fermi");
        } else if rand_num.contains(guess_chars[i]) {
            clues.push("Pico");
        }
    }

    if clues.is_empty() {
        println!("Bagels")
    } else {
        clues.sort();
        println!("{}", clues.join(" "));
    }
}

fn get_num() -> String {
    let mut rng = rand::rng();
    let mut nums: Vec<i8> = (0..9).collect();
    nums.shuffle(&mut rng);

    nums[0..3].iter().map(|n| n.to_string()).collect()
}
