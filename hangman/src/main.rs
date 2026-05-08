use rand::seq::IndexedRandom;
use std::io;
use std::io::Write;
fn main() {

    //all of this is getting the game set up to play

    let mut rng = rand::rng();
    let hang_list = vec!["Guitar", "Nerds", "Prophet", "String", "Rust", "Coding", "BYUI", "Dingus", "Hangman"];
    let word = hang_list.choose(&mut rng).expect("hang_list is empty");
    //println!("Word is: {}", word)

    struct Letter {
        letter: char,
        guessed: bool,
    }

    struct FormedWord {
        letters: Vec<Letter>,
    }

    let mut formed = FormedWord {
        letters: word.chars().map(|c| Letter {
            letter: c,
            guessed: false,
        }).collect(),
    };

    fn display(f: &FormedWord) {
        for lett in &f.letters {
            if lett.guessed == true {
                print!("{} ", lett.letter);
            }
            else {
                print!("_ ");
            }
        }
    }

    fn get_guess() -> char {
        println!("");
        print!("Enter a guess: ");
        io::stdout().flush().unwrap();
        let mut usertext = String::new();
        io::stdin().read_line(&mut usertext).expect("Failed to read input");
    return usertext.trim().chars().next().expect("No character entered").to_lowercase().next().unwrap();
    }

    fn check_word(f: &FormedWord) -> bool {
        for lett in &f.letters {
            if lett.guessed == false {
                return false;
            }
        }
        return true;
    }

    let mut chances = 6;

    let mut marker = true;

    let mut over = false;

    /* for ch in formed.letters {
        println!("The letter is {}", ch.letter);
    } */

    //now to make the game

    //display(&formed);

    while over == false {
        marker = true;
        match chances {
            6 => println!("You have 6 chances left - no wrong guesses yet!"),
            5 => println!("You have 5 chances left - got one wrong, but you'll be fine."),
            4 => println!("You have 4 chances left - better start guessing right!"),
            3 => println!("You have 3 chances left - don't mess this up!"),
            2 => println!("You have 2 chances left - scared you don't know the word?"),
            1 => println!("You have 1 chance left - you might be cooked..."),
            _ => println!("")
        }
        display(&formed);
        let mut guess = get_guess();
        //println!("You entered {}", guess);
        for lett in &mut formed.letters {
            if guess == lett.letter.to_lowercase().next().unwrap() {
                lett.guessed = true;
                marker = false;
            }
        }

        if marker == true {
            chances -= 1;
        }

        if chances == 0 {
            over = true;
            println!("You lost :((\nThe correct word was: {}", word)
        }

        if check_word(&formed) == true {
            over = true;
            println!("You won!\nThe correct word was: {}", word)
        }
    }
}
