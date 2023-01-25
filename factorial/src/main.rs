use std::io;

fn main() {
    println!("Please enter a number: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<u64>().unwrap();

    println!("{}! = {}", input, factorial(input));
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}
