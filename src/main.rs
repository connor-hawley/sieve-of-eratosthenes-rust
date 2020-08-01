use std::io::stdin;

fn main() {
    read_line();
}

fn read_line() {
    let mut input = String::new();
    println!("Primes up to what number should be discovered?");
    match stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
}