#[allow(unused_variables)]
#[allow(unused_imports)]
use std::fs::File;
use std::io::{self, BufRead, Write, stdin, stdout};
use std::path::Path;
use colored::{Colorize};
use rand::Rng; // 0.8.0

const GUESSES_ALLOWED: usize = 7;

/// Get the words
fn read_words() -> (Vec<String>, usize) {
    let mut word_list = Vec::<String>::new();

    if let Ok(lines) = read_lines("./words.txt") {
        for line in lines {
            if let Ok(word) = line {
                if looks_like_a_word(&word) {
                    word_list.push(word);
                } else {
                    println!("Ignoring {}, which doesn't look like a word", word);
                }
            }
        }
    }

    let word_length: usize = word_list.first().expect("Empty word list!").len();
    for word in word_list.iter() {
        assert_eq!(word.len(), word_length);
    }

    return (word_list, word_length);
}


fn looks_like_a_word(word: &String) -> bool {
    word.chars().all(char::is_alphabetic) &&
    word.chars().all(char::is_lowercase)
}

/// Looks like a word, matches target length, and in word list
fn is_valid_guess(guess: &String, words: &Vec<String>, word_length: usize) -> bool {
    looks_like_a_word(guess) &&
    guess.len() == word_length &&
    words.contains(&guess)
}

/// Color-coded string indicating how guess matches target
fn compare_words(guess: &String, target: &String) -> String {
    let mut s = Vec::<String>::new();

    for (v, w) in guess.chars().zip(target.chars()) {
        let next: String = (
            if v == w {
                v.to_string().green().to_string()
            } else if target.contains(v) {
                v.to_string().yellow().to_string()
            } else {
                v.to_string().to_string()
            }
        );
        s.push(next);
    }
    return s.join("");
}


/// String of this many spaces
fn blank_string(word_length: usize) -> String {
    (0..word_length).map(|_| ' ').collect()
}


/// Play a game
fn main() {
    let (word_list, word_length) = read_words();

    let secret_words = {
        let idx0 = rand::thread_rng().gen_range(0..word_list.len());
        let idx1 =  rand::thread_rng().gen_range(0..word_list.len());
        (word_list[idx0].clone(), word_list[idx1].clone())
    };

    let mut guesses_allowed: usize = GUESSES_ALLOWED;
    let mut guessed_words = (false, false);

    println!();
    loop {
        print!("Enter a {}-letter word: ", word_length); stdout().flush();
        let mut guess = String::new();
        stdin().read_line(&mut guess);
        guess.pop(); // Remove newline character

        if is_valid_guess(&guess, &word_list, word_length) {
            guesses_allowed -= 1;

            let comp = (
                compare_words(&guess, &secret_words.0),
                compare_words(&guess, &secret_words.1)
            );

            print!(
                "{}   {}   ",
                if guessed_words.0 {blank_string(word_length)} else {comp.0},
                if guessed_words.1 {blank_string(word_length)} else {comp.1},
            );

            if guess == secret_words.0 {
//                 println!("You guessed the zeroth secret word!");
                guessed_words.0 = true;
            }
            if guess == secret_words.1 {
//                 println!("You guessed the first secret word!");
                guessed_words.1 = true;
            }
        } else {
            println!("That's not a word!");
        }
        if guessed_words.0 && guessed_words.1 {
            println!("You win!");
            break;
        }
        if guesses_allowed < 1 {
            println!("You lose!");
            break;
        }
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
