use colored::*;
use rand::{self, Rng};
use std::cmp::Ordering;
use std::io;

fn play_game() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let my_guess = rand::thread_rng().gen_range(0..3);
    println!("{}", "Guess the number!".blue().bold());
    println!("{}", "Please input your guess.".bright_green());

    loop {
        let mut your_guess: String = String::new();

        io::stdin()
            .read_line(&mut your_guess)
            .expect("Failed to read line");
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
