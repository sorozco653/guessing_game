use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Next Game!");
    println!("Guess the Number Between 1 and 100!");

    let range = 1..=100;
    let secret_number = rand::thread_rng().gen_range(range);

    println!("Input your guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let number_guess = guess.trim().parse::<u32>().expect(" I guess lets try error handling?");

    println!("You have guess {guess}");

    match number_guess.cmp(&secret_number) {
        Ordering::Greater => println!("Too Big Fool"),
        Ordering::Less => println!("Too small Imbecile"),
        Ordering::Equal => println!("Exactly That!!")
    };

    println!("The correct number is {secret_number}")
}
