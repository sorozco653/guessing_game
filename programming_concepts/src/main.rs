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
}
