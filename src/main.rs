use std::io;
use std::cmp::{Ordering, PartialEq};
use std::io::Write;
use rand::Rng;

const INFINITE_GUESSES: u32 = 0;

const SCORE_PER_GUESS_POINT: u32 = 100;
const MEDIUM_MULTIPLIER: u32 = 2;
const HARD_MULTIPLIER: u32 = 4;


fn main() {


    display_main_menu();
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn display_main_menu() {
    let mut game_difficulty = Difficulty::Easy;
    let mut guess_difficulty = Difficulty::Easy;

    loop {
        clear_screen();
        print_title();

        println!("Main menu: ");
        println!("1. Play");
        println!("2. Settings");
        println!("3. Quit");
        println!();
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

        match user_choice {
            1 => run_game(&game_difficulty, &guess_difficulty),
            2 => display_settings(&mut game_difficulty, &mut guess_difficulty),
            3 => break,
            _ => {
                println!("Your option is invalid");
                continue;
            },
        };
    }
}

fn print_title() {
    println!("      ___           ___           ___           ___           ___                       ___           ___                    ___           ___           ___           ___     ");
    println!("     /\\  \\         /\\__\\         /\\  \\         /\\  \\         /\\  \\          ___        /\\__\\         /\\  \\                  /\\  \\         /\\  \\         /\\__\\         /\\  \\    ");
    println!("    /::\\  \\       /:/  /        /::\\  \\       /::\\  \\       /::\\  \\        /\\  \\      /::|  |       /::\\  \\                /::\\  \\       /::\\  \\       /::|  |       /::\\  \\   ");
    println!("   /:/\\:\\  \\     /:/  /        /:/\\:\\  \\     /:/\\ \\  \\     /:/\\ \\  \\       \\:\\  \\    /:|:|  |      /:/\\:\\  \\              /:/\\:\\  \\     /:/\\:\\  \\     /:|:|  |      /:/\\:\\  \\  ");
    println!("  /:/  \\:\\  \\   /:/  /  ___   /::\\~\\:\\  \\   _\\:\\~\\ \\  \\   _\\:\\~\\ \\  \\      /::\\__\\  /:/|:|  |__   /:/  \\:\\  \\            /:/  \\:\\  \\   /::\\~\\:\\  \\   /:/|:|__|__   /::\\~\\:\\  \\ ");
    println!(" /:/__/_\\:\\__\\ /:/__/  /\\__\\ /:/\\:\\ \\:\\__\\ /\\ \\:\\ \\ \\__\\ /\\ \\:\\ \\ \\__\\  __/:/\\/__/ /:/ |:| /\\__\\ /:/__/_\\:\\__\\          /:/__/_\\:\\__\\ /:/\\:\\ \\:\\__\\ /:/ |::::\\__\\ /:/\\:\\ \\:\\__\\");
    println!(" \\:\\  /\\ \\/__/ \\:\\  \\ /:/  / \\:\\~\\:\\ \\/__/ \\:\\ \\:\\ \\/__/ \\:\\ \\:\\ \\/__/ /\\/:/  /    \\/__|:|/:/  / \\:\\  /\\ \\/__/          \\:\\  /\\ \\/__/ \\/__\\:\\/:/  / \\/__/~~/:/  / \\:\\~\\:\\ \\/__/");
    println!("  \\:\\ \\:\\__\\    \\:\\  /:/  /   \\:\\ \\:\\__\\    \\:\\ \\:\\__\\    \\:\\ \\:\\__\\   \\::/__/         |:/:/  /   \\:\\ \\:\\__\\             \\:\\ \\:\\__\\        \\::/  /        /:/  /   \\:\\ \\:\\__\\  ");
    println!("   \\:\\/:/  /     \\:\\/:/  /     \\:\\ \\/__/     \\:\\/:/  /     \\:\\/:/  /    \\:\\__\\         |::/  /     \\:\\/:/  /              \\:\\/:/  /        /:/  /        /:/  /     \\:\\ \\/__/  ");
    println!("    \\::/  /       \\::/  /       \\:\\__\\        \\::/  /       \\::/  /      \\/__/         /:/  /       \\::/  /                \\::/  /        /:/  /        /:/  /       \\:\\__\\    ");
    println!("     \\/__/         \\/__/         \\/__/         \\/__/         \\/__/                     \\/__/         \\/__/                  \\/__/         \\/__/         \\/__/         \\/__/    ");
}

fn run_game(selected_game_difficulty: &Difficulty, selected_guess_difficulty: &Difficulty) {
    clear_screen();
    println!("Guess the number!");

    let max_authorized_guesses = get_max_authorized_guesses(&selected_guess_difficulty);
    let mut current_max_rand_range = get_default_max_rand_range(&selected_game_difficulty);

    let mut score: u32 = 0;

    loop {
        print_game_settings(&selected_game_difficulty, &selected_guess_difficulty, &current_max_rand_range);
        
        let game_guess_count = start_guess_game(max_authorized_guesses, current_max_rand_range);

        let compute_score = compute_score(game_guess_count, max_authorized_guesses, &selected_game_difficulty);

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

        current_max_rand_range += get_increment_max_rand_range(&selected_game_difficulty);
    }
}

fn print_game_settings(selected_game_difficulty: &Difficulty, selected_guess_difficulty: &Difficulty, current_max_rand_range: &u32) {
    println!("=== Start Guessing =======================================================================");
    
    println!("Selected Game Difficulty: {}", get_difficulty_name(&selected_game_difficulty));
    println!("  -> Secret number randomly selected between 1 and {} (default: 1 and {})", current_max_rand_range, get_default_max_rand_range(&selected_game_difficulty));
    
    println!("Selected Guess Difficulty: {}", get_difficulty_name(&selected_guess_difficulty));
    if (*selected_guess_difficulty == Difficulty::Easy) {
        println!("  -> Infinite number of guess)");
    } else {
        println!("  -> {} guesses", get_max_authorized_guesses(&selected_guess_difficulty))
    }
    println!("==========================================================================================");
}

fn display_settings(selected_game_difficulty: &mut Difficulty, selected_guess_difficulty: &mut Difficulty) {
    loop {
        clear_screen();
        println!("Settings: ");
        println!("1. Game Difficulty");
        println!("2. Guess Difficulty");
        println!("other. Return");
        println!();
        print!("Your choice: ");
        io::stdout().flush().expect("Failed to write");

        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");

        let user_choice: u32 = match user_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => break
        };

        match user_choice {
            1 => {
                choose_a_game_difficulty_menu(selected_game_difficulty);
            },
            2 => {
                choose_a_guess_difficulty_menu(selected_guess_difficulty);
            },
            _ => break,
        };
    }
}

fn choose_a_game_difficulty_menu(selected_game_difficulty: &mut Difficulty) {
    clear_screen();
    println!("Game Difficulty:");

    println!();

    print_game_difficulty_description();

    println!();
    println!();

    loop {
        clear_screen();
        println!("Guess Difficulty:");

        println!();

        print_game_difficulty_description();

        println!();
        println!();

        print_choose_difficulty_options(&selected_game_difficulty);
        match get_user_difficulty_choice() {
            Some(difficulty) => *selected_game_difficulty = difficulty,
            None => break
        }
    }
}

fn print_game_difficulty_description() {
    println!("Easy");
    println!("  -> Start with a guess between 1 and {}", get_default_max_rand_range(&Difficulty::Easy));
    println!("  -> Increment upper guess bound by {}", get_increment_max_rand_range(&Difficulty::Easy));
    println!("  -> Game multiplier x{}", get_multiplier(&Difficulty::Easy));

    println!();

    println!("Medium");
    println!("  -> Start with a guess between 1 and {}", get_default_max_rand_range(&Difficulty::Medium));
    println!("  -> Increment upper guess bound by {}", get_increment_max_rand_range(&Difficulty::Medium));
    println!("  -> Game multiplier x{}", get_multiplier(&Difficulty::Medium));

    println!();

    println!("Hard");
    println!("  -> Start with a guess between 1 and {}", get_default_max_rand_range(&Difficulty::Hard));
    println!("  -> Increment upper guess bound by {}", get_increment_max_rand_range(&Difficulty::Hard));
    println!("  -> Game multiplier x{}", get_multiplier(&Difficulty::Hard));
}

fn choose_a_guess_difficulty_menu(selected_guess_difficulty: &mut Difficulty) {
    loop {
        clear_screen();
        println!("Guess Difficulty:");

        println!();

        print_guess_difficulty_description();

        println!();
        println!();
        
        print_choose_difficulty_options(&selected_guess_difficulty);
        match get_user_difficulty_choice() {
            Some(difficulty) => *selected_guess_difficulty = difficulty,
            None => break
        }
    }
}

fn print_guess_difficulty_description() {
    println!("Easy -> No maximum guess");
    println!("Medium -> {} maximum guesses", get_max_authorized_guesses(&Difficulty::Medium));
    println!("Hard -> {} maximum guesses", get_max_authorized_guesses(&Difficulty::Hard));
}

fn print_choose_difficulty_options(selected_difficulty: &Difficulty) {
    println!("Choose a difficulty:");
    println!("1. Easy {}", get_selected_tag(&selected_difficulty, &Difficulty::Easy));
    println!("2. Normal {}", get_selected_tag(&selected_difficulty, &Difficulty::Medium));
    println!("3. Hard {}", get_selected_tag(&selected_difficulty, &Difficulty::Hard));
    println!("other. Return");
    println!();
    print!("Your choice: ");
    io::stdout().flush().expect("Failed to write");
}

fn get_user_difficulty_choice() -> Option<Difficulty> {
    let mut user_choice = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read line");

    let user_choice: u32 = match user_choice.trim().parse() {
        Ok(num) => num,
        Err(_) => return None
    };

    match user_choice {
        1 => Some(Difficulty::Easy),
        2 => Some(Difficulty::Medium),
        3 => Some(Difficulty::Hard),
        _ => None,
    }
}

fn get_selected_tag<'a>(selected_difficulty: &Difficulty, difficulty: &Difficulty) -> &'a str {
    if *selected_difficulty == *difficulty {
        "(selected)"
    } else {
        ""
    }
}

fn start_guess_game(max_guesses: u32, max_rand_range: u32) -> u32 {
    let secret_number = generate_secret_number(max_rand_range);
    let mut guess_count = 0;

    loop {
        println!();
        if has_max_guess_defined(max_guesses) {
            print!("{} guess(es) left. ", max_guesses - guess_count);
            
            if has_reached_max_guess(max_guesses, guess_count) {
                break;
            }
        }

        let guess = ask_guess(max_rand_range);
        guess_count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("  -> Too small!"),
            Ordering::Greater => println!("  -> Too big!"),
            Ordering::Equal => {
                println!();
                println!("*** You win in {} guesses! ***", guess_count);
                return guess_count;
            },
        }
    }

    println!();
    println!("~~~ You lose! You guessed too many guesses :( ~~~");
    guess_count
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
                println!("/!\\ Please enter a valid number");
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
    if max_authorized_guesses == INFINITE_GUESSES {
        return SCORE_PER_GUESS_POINT;
    }

    let score = (max_authorized_guesses - guess_count) * SCORE_PER_GUESS_POINT;
    
    score * get_multiplier(&game_difficulty)
}

fn get_multiplier(game_difficulty: &Difficulty) -> u32 {
    match game_difficulty {
        Difficulty::Easy => 1,
        Difficulty::Medium => MEDIUM_MULTIPLIER,
        Difficulty::Hard => HARD_MULTIPLIER
    }
}

fn get_difficulty_name(difficulty: &Difficulty) -> &str {
    match difficulty {
        Difficulty::Easy => "Easy",
        Difficulty::Medium => "Medium",
        Difficulty::Hard => "Hard",
    }
}

#[derive(PartialEq, Eq)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}