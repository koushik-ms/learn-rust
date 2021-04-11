use std::io;

fn main() {
    let mut ns1 = String::new();
    let mut ns2 = String::new();
    println!("Enter two numbers: ");
    io::stdin().read_line(&mut ns1).ok().expect("Unable to read number 1");
    io::stdin().read_line(&mut ns2).ok().expect("Unable to read number 2");

    let num1 : i32 = ns1.trim().parse().ok().expect("Input 1 is not a valid number");
    let num2 : i32 = ns2.trim().parse().ok().expect("Input 2 is not a valid number");
    println!("\nSum of {} and {} is {}. {} + {} = {}", ns1.trim(), ns2.trim(), num1+num2, num1, num2, num1+num2);
}