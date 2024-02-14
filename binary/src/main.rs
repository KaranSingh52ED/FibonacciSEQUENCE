use std::io;
use library;
fn main() {
    println!("Enter a valid number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input number");
    let n: u32 = input.trim().parse().expect("Please enter a valid number");

    let Seq = library::fibonacciSequence(n);
    println!("Fibonacci sequence up to {} terms: {:?}", n, Seq);
}
