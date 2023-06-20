use std::io;

fn main() {
    println!("Welcome to Sergio's Gussing Game!!!");
    println!("Please Input your Guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to Read??");

    print!("You have guess: {guess}")

}
