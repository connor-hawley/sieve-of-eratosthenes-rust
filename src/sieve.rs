pub struct Sieve {
    pub evaluation_limit: usize
}

pub trait Sift {
    fn sift(&self) -> Vec<usize>;
}

impl Sieve {
    pub fn new(evaluation_limit: usize) -> Sieve {
        Sieve {
            evaluation_limit
        }
    }
}

impl Sift for Sieve {
    fn sift(&self) -> Vec<usize> {
        let evaluation_limit = self.evaluation_limit;
        
        let prime_minimum = 2;
        let mut candidates = vec![true; evaluation_limit];
        candidates[0] = false; // 1 is not prime
        let divisor_maximum = evaluation_limit;  // TODO: abstract this out
        for divisor in prime_minimum..=divisor_maximum {
            for dividend_idx in divisor..evaluation_limit {
                let is_prime = candidates[dividend_idx];
                let dividend = dividend_idx + 1;
                if is_prime && evenly_divide(dividend, divisor) {
                    candidates[dividend_idx] = false;
                }
            }
        }

        candidates
            .iter()
            .enumerate()
            .filter(|(_, &is_prime)| is_prime)
            .map(|(prime_idx, _)| prime_idx + 1)
            .collect()
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
        let evaluation_limit = 11;
        let sieve = Sieve::new(evaluation_limit);
        let primes = sieve.sift();
        let expected_primes: Vec<usize> = vec![2, 3, 5, 7, 11];
        assert_eq!(expected_primes, primes);
    }

    #[test]
    fn it_handles_lower_bound() {
        let evaluation_limit = 2;
        let sieve = Sieve::new(evaluation_limit);
        let primes = sieve.sift();
        let expected_primes: Vec<usize> = vec![2];
        assert_eq!(expected_primes, primes);
    }
}