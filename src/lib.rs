use wasm_bindgen::prelude::*;

// TODO: use linear memory with serde and buffer to write primes to something js can understand
#[wasm_bindgen]
pub fn sift(evaluation_limit: usize, smart_divisor_maximum: bool) -> Vec<usize> {    
    let prime_minimum = 2;
    let mut candidates = vec![true; evaluation_limit];
    candidates[0] = false; // 1 is not prime
    let divisor_maximum = get_divisor_maximum(evaluation_limit, smart_divisor_maximum);
    for divisor in prime_minimum..=divisor_maximum {
        for dividend_idx in divisor..evaluation_limit {
            let is_prime = candidates[dividend_idx];
            let dividend = dividend_idx + 1;
            if is_prime && evenly_divide(dividend, divisor) {
                candidates[dividend_idx] = false;
            }
        }
    }

    let final_primes: Vec<usize> = candidates
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(prime_idx, _)| prime_idx + 1)
        .collect();
    final_primes
}

fn get_divisor_maximum(evaluation_limit: usize, smart_divisor_maximum: bool) -> usize {
    match smart_divisor_maximum {
        false => evaluation_limit,
        true => {
            let evaluation_limit = evaluation_limit as f32;
            evaluation_limit.sqrt().ceil() as usize
        },
    }
}

fn evenly_divide(dividend: usize, divisor: usize) -> bool {
    match dividend % divisor {
        0 => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sifts_first_primes() {
        let expected_primes: Vec<usize> = vec![2, 3, 5, 7, 11];
        assert_eq!(expected_primes, sift(11, true));
    }

    #[test]
    fn it_handles_lower_bound() {
        let expected_primes: Vec<usize> = vec![2];
        assert_eq!(expected_primes, sift(2, true));
    }

    #[test]
    fn it_gets_dumb_divisor() {
        assert_eq!(17, get_divisor_maximum(17, false));
    }

    #[test]
    fn it_gets_smart_divisor() {
        assert_eq!(5, get_divisor_maximum(18, true));
    }
}