// Gets the first n prime numbers
pub fn get_n_primes(n: usize) -> Vec<i128> {
    let mut primes = Vec::new();
    let mut num = 2;

    while primes.len() < n as usize {
        let mut is_prime = true;
        for i in 2..num {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(num);
        }
        num += 1;
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests whether get_n_primes correctly generates the first 12 prime numbers
    #[test]
    fn get_12_primes() {
        let result = get_n_primes(12);
        assert_eq!(result, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]);
    }
}
