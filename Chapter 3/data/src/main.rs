fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    // addition
    let sum = 5 + 10;
    println!("{sum}");

    // subtraction
    let difference = 95.5 - 4.3;

    println!("{difference}");

    // multiplication
    let product = 4 * 30;

    println!("{product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{quotient}");
    println!("{truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");

    // Compounds
    let tup: (u32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let m = tup.1;
    println!("The value of x is {x}. The value of y is {y}. The value of z is {z}");
    println!("The value of m is {m}.");

    // Arrays
    let a = [1, 2, 3, 4, 5];
}
