use std::io;

fn main() {
    let mut ns1 = String::new();
    let mut ns2 = String::new();
    println!("Enter numbers: ");
    io::stdin().read_line(&mut ns1).ok().expect("Unable to read line 1");
    io::stdin().read_line(&mut ns2).ok().expect("Unable to read line 2");

    let vector = ns2
        .split(' ')               // Tokenize
        .map(|x| {
            println!("Parsing {}", x);
            x.trim().parse().ok().expect("Input is not a valid number")
        })                        // Parse to integer
        .collect::<Vec<usize>>(); // Return as a new Vec<usize>
    println!("Data {:?}", vector);     // Print result
    let ans: usize = vector.iter().sum();
    println!("Sum {:?}", ans);     // Print result
}
