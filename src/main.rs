use std::io;
 
mod sieve;
use sieve::Sieve;
use sieve::Sift;

fn main() {
    let evaluation_limit = get_evaluation_limit_from_user();
    let sieve = Sieve::new(evaluation_limit);
    println!("Running sieve for numbers up to {}", evaluation_limit);
    let primes = sieve.sift();
    println!("Primes up until {}", evaluation_limit);
    for prime in primes.iter() {
        println!("{}", prime);
    }
}

fn get_evaluation_limit_from_user() -> usize {
    match read_line() {
        Ok(evaluation_limit) => evaluation_limit as usize,
        Err(invalid_input) => {
            println!("input {} was not a valid integer", invalid_input);
            get_evaluation_limit_from_user()
        }
    }
}

fn read_line() -> Result<i32, String> {
    let mut input = String::new();
    println!("Primes up to what number should be discovered?");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("parsing string: \"{}\" as integer", input.trim());
            parse_integer(input.trim().to_string())
        },
        Err(_) => {
            println!("could not read line");
            Err(input)
        }
    }
}

fn parse_integer(string_to_convert: String) -> Result<i32, String> {
    match string_to_convert.parse::<i32>() {
        Ok(evaluation_limit) => {
            println!("parsed string {} as integer: {}", string_to_convert, evaluation_limit);
            Ok(evaluation_limit)
        },
        Err(e) => {
            println!("failed to parse string {} as integer: {}", string_to_convert, e);
            Err(string_to_convert)
        }
    }
}