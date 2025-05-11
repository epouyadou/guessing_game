use std::io;
use std::cmp::Ordering;
use std::io::Write;
use rand::Rng;

const INFINITE_GUESSES: u32 = 0;

const SCORE_PER_GUESS_POINT: u32 = 100;
const MEDIUM_MULTIPLIER: u32 = 2;
const HARD_MULTIPLIER: u32 = 4;


fn main() {
    println!("Guess the number!");
    
    let game_difficulty = choose_a_game_difficulty_menu();
    let guess_difficulty = choose_a_guess_difficulty_menu();
    
    let max_authorized_guesses = get_max_authorized_guesses(&guess_difficulty);
    let mut current_max_rand_range = get_default_max_rand_range(&game_difficulty);
    
    let mut score: u32 = 0;

    loop {
        println!("=== Start Guessing ====================================");
        
        let game_guess_count = start_guess_game(max_authorized_guesses, current_max_rand_range);
        
        let compute_score = compute_score(game_guess_count, max_authorized_guesses, &game_difficulty);
        
        println!("Score: {} ({} + {})", score + compute_score, score, compute_score);

        score += compute_score;
        
        println!("Do you want to play again? (Y/n)");
        
        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");
        
        let should_quit = user_choice.trim().to_lowercase() == "n";
        if should_quit {
            break;
        }
        
        current_max_rand_range += get_increment_max_rand_range(&game_difficulty);
    }
}

fn choose_a_guess_difficulty_menu() -> Difficulty {
    println!("Configure the number of guess");
    print!("Easy = Infinite guess | ");
    print!("Medium = {} guesses | ", get_max_authorized_guesses(&Difficulty::Medium));
    println!("Hard = {} guesses", get_max_authorized_guesses(&Difficulty::Hard));

    choose_a_difficulty_menu()
}

fn choose_a_game_difficulty_menu() -> Difficulty {
    println!("Configure the game difficulty");
    print!("Easy -> between 1 and 100 | ");
    print!("Medium -> between 1 and 1000 | ");
    println!("Hard -> between 1 and 10000");

    choose_a_difficulty_menu()
}

fn choose_a_difficulty_menu() -> Difficulty {
    loop {
        println!("Choose a difficulty:");
        println!("1. Easy");
        println!("2. Normal");
        println!("3. Hard");
        print!("Your choice: ");
        io::stdout().flush().expect("Failed to write");

        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");

        let user_choice: u32 = match user_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let difficulty =  match user_choice {
            1 => Difficulty::Easy,
            2 => Difficulty::Medium,
            3 => Difficulty::Hard,
            _ => {
                println!("Your option is invalid");
                continue;
            },
        };
        return difficulty;
    }
}

fn start_guess_game(max_guesses: u32, max_rand_range: u32) -> u32 {
    let secret_number = generate_secret_number(max_rand_range);
    let mut guess_count = 0;

    loop {
        if has_max_guess_defined(max_guesses) {
            print!("{} guess(es) left. ", max_guesses - guess_count);
            
            if has_reached_max_guess(max_guesses, guess_count) {
                break;
            }
        }

        let guess = ask_guess(max_rand_range);
        guess_count += 1;
        
        print!("You guessed: {guess}. ");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("This is too small!"),
            Ordering::Greater => println!("This is too big!"),
            Ordering::Equal => {
                println!("You win in {} guesses!", guess_count);
                return guess_count;
            },
        }
    }

    println!("You lose! You guessed too many guesses :(");
    return guess_count;
}

fn ask_guess(max_rand_range: u32) -> u32 {
    loop {
        println!("Guess the secret number between 1 & {}", max_rand_range);
        print!("Your guess : ");
        io::stdout().flush().expect("Failed to write");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
    }
}

fn has_max_guess_defined(max_guesses: u32) -> bool {
    max_guesses > INFINITE_GUESSES
}

fn has_reached_max_guess(max_guesses: u32, current_guess_count: u32) -> bool {
    max_guesses - current_guess_count <= 0
}

fn generate_secret_number(max_rand_range: u32) -> u32 {
    rand::thread_rng().gen_range(1..=max_rand_range)
}

fn get_default_max_rand_range(difficulty: &Difficulty) -> u32 {
    match difficulty {
        Difficulty::Easy => 100,
        Difficulty::Medium => 1000,
        Difficulty::Hard => 10000,
    }
}

fn get_increment_max_rand_range(difficulty: &Difficulty) -> u32 {
    match difficulty {
        Difficulty::Easy => 25,
        Difficulty::Medium => 250,
        Difficulty::Hard => 2500,
    }
}

fn get_max_authorized_guesses(difficulty: &Difficulty) -> u32 {
    match difficulty {
        Difficulty::Easy => INFINITE_GUESSES,
        Difficulty::Medium => 20,
        Difficulty::Hard => 10,
    }
}

fn compute_score(guess_count: u32, max_authorized_guesses: u32, game_difficulty: &Difficulty) -> u32 {
    // If user can make infinite guesses, return a fixed score.
    if (max_authorized_guesses == INFINITE_GUESSES) {
        return SCORE_PER_GUESS_POINT;
    }
    
    let score = (max_authorized_guesses - guess_count) * SCORE_PER_GUESS_POINT;
    
    match game_difficulty {
        Difficulty::Easy => score,
        Difficulty::Medium => score * MEDIUM_MULTIPLIER,
        Difficulty::Hard => score * HARD_MULTIPLIER
    }
}

enum Difficulty {
    Easy,
    Medium,
    Hard,
}