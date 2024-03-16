use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod range_selector;

fn play_game() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", "Welcome to the guessing game!".blue().bold());
    println!(
        "{}",
        "Please input the range of the number. default 0-3".bright_green()
    );
    let range = range_selector::get_range();
    println!("Range: {:?}", range);

    let my_guess = rand::thread_rng().gen_range(range.0..range.1);
    println!(
        "{} My Guess: {}",
        "Guess the number!".blue().bold(),
        my_guess
    );
    println!("{}", "Please input your guess.".bright_green());

    loop {
        let your_guess = range_selector::read_input();
        let your_guess: u32 = match your_guess.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };
        match your_guess.cmp(&my_guess) {
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too high.".yellow()),
            Ordering::Less => println!("{}", "Too low.".yellow()),
        }
    }
}
fn main() {
    play_game();
    println!("Would you like to play again? [y/n]");
    let mut play_again: String = String::new();
    io::stdin()
        .read_line(&mut play_again)
        .expect("Failed read the line!");
    println!("{}", play_again);
    if play_again.trim() == "y" {
        println!("OK");
        main();
    }
}
