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
    
    let mut primes_list = String::new();
    let list_delimeter = ", ";
    for prime in primes.iter() {
        primes_list.push_str(&prime.to_string());
        primes_list.push_str(list_delimeter);
    }
    println!("{}", primes_list);
}

fn get_evaluation_limit_from_user() -> usize {
    match read_line() {
        Ok(evaluation_limit) => evaluation_limit,
        Err(invalid_input) => {
            println!("input {} was not a valid integer", invalid_input);
            get_evaluation_limit_from_user()
        }
    }
}

fn read_line() -> Result<usize, String> {
    let mut input = String::new();
    println!("Primes up to what number should be discovered?");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("parsing string: \"{}\" as unsigned integer", input.trim());
            parse_integer(input.trim().to_string())
        },
        Err(_) => {
            println!("could not read line");
            Err(input)
        }
    }
}

fn parse_integer(string_to_convert: String) -> Result<usize, String> {
    match string_to_convert.parse::<usize>() {
        Ok(evaluation_limit) => {
            println!("parsed string {} as unsigned integer: {}", string_to_convert, evaluation_limit);
            Ok(evaluation_limit)
        },
        Err(e) => {
            println!("failed to parse string {} as unsigned integer: {}", string_to_convert, e);
            Err(string_to_convert)
        }
    }
}