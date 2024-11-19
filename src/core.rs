use std::fmt::Display;

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

#[cfg(test)]
mod tests {
    use crate::core::to_names;

    // Tests whether to_names works correctly, using one of devine's canonical examples.
    // "ACC 21450 flour sugar apples apples oranges cherries"
    #[test]
    pub fn test_to_names() {
        let prime_product = 21450;
        let names = [
            "flour",
            "sugar",
            "apples",
            "apple-cake",
            "oranges",
            "cherries",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let result = ["flour", "sugar", "apples", "apples", "oranges", "cherries"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        assert_eq!(to_names(prime_product, &names), result);
    }

    // Tests whether to_rules works correctly, using one of devine's canonical examples.
    // ":: 7/30 flour.2 sugar.3 apples.5 > apple-cake.7"
    #[test]
    pub fn test_to_rule() {
        let frac = super::Frac::new(7, 30);
        let names = [
            "flour",
            "sugar",
            "apples",
            "apple-cake",
            "oranges",
            "cherries",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let left = ["flour", "sugar", "apples"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let right = ["apple-cake"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let rule = frac.to_rule(&names);

        assert_eq!(rule.left, left);
        assert_eq!(rule.right, right);
    }
}
