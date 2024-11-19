use std::fmt::{Display, Pointer};

use crate::{
    ast::{Name, Rule},
    util,
};

// Executes one step of the fractran program,
// returning a new accumulator (state) and the fraction that was applied.
// If no fraction could be multiplied, the second result is None.
pub fn step(acc: i128, fracs: &[Frac]) -> (i128, Option<&Frac>) {
    let mut new_acc = acc;
    let mut applied_frac = None;

    for frac in fracs {
        if new_acc % frac.den == 0 {
            new_acc = new_acc * frac.num / frac.den;
            applied_frac = Some(frac);
            break;
        }
    }

    (new_acc, applied_frac)
}

#[derive(Debug, Clone)]
pub struct Frac {
    pub num: i128,
    pub den: i128,
}

impl Frac {
    // Creates a new fraction, with a numerator and a denominator.
    pub fn new(num: i128, den: i128) -> Frac {
        Frac { num, den }
    }

    // Converts a fraction to a rule.
    pub fn to_rule(&self, names: &[Name]) -> Rule {
        Rule::new(to_names(self.den, names), to_names(self.num, names))
    }
}

impl Display for Frac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{}/{}", self.num, self.den).fmt(f)
    }
}

// Converts a prime product to the list of names that it corresponds to,
// according to the ordered list of names.
pub fn to_names(prime_product: i128, names: &[Name]) -> Vec<Name> {
    let primes = util::get_n_primes(names.len());

    let mut new_prime_product = prime_product;
    let mut new_names = vec![];
    let mut i = primes.iter().zip(names);

    let mut a = i.next();
    while let Some((prime, name)) = a {
        if new_prime_product % prime == 0 {
            new_prime_product = new_prime_product / prime;
            new_names.push(name.clone());
        } else {
            a = i.next();
        }
    }

    new_names
}
