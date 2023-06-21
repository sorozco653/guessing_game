use std::io;

fn main() {
    // Let + Mut
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Print Hours in Seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 10;

    let x = x + 1; // The previous X is 10 so this will equal to 11

    {
        let x = x * 2; // Previous value was 11 so this means 11 * 2 = 22
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // Variable are block scope so the last value of x was 11

    // Tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    invalid_arr()
}

fn invalid_arr () {
    let a = [1,2,3,4,5];

    println!("Please Enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index = index
        .trim()
        .parse::<usize>()
        .expect("Index was not a number dude");

    let element = a[index];

    println!("Element: {element}, Index: {index}")
}