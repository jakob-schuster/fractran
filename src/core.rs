use std::fmt::{Display, Pointer};

use crate::ast::{Name, Rule};

#[derive(Debug)]
pub struct Prog {
    acc: i128,
    fracs: Vec<Frac>,
}

impl Prog {
    pub fn new(acc: i128, fracs: Vec<Frac>) -> Prog {
        Prog { acc, fracs }
    }

    pub fn eval(&self) -> i128 {
        let mut new_acc = self.acc;
        let mut used = true;
        while used {
            used = false;
            for frac in &self.fracs {
                if new_acc % frac.den == 0 {
                    new_acc = new_acc * frac.num / frac.den;
                    used = true;
                    break
                }
            }
        }
        new_acc
    }

    pub fn step(&self) -> (Prog, Option<Frac>) {
        let mut new_acc = self.acc;
        let mut applied_frac = None;

        for frac in &self.fracs {
            if new_acc % frac.den == 0 {
                new_acc = new_acc * frac.num / frac.den;
                applied_frac = Some(frac.clone());
                break
            }
        }

        (Prog::new(new_acc, self.fracs.clone()), applied_frac)
    }
}

pub fn step(acc: i128, fracs: &[Frac]) -> (i128, Option<&Frac>) {
    let mut new_acc = acc;
    let mut applied_frac = None;

    for frac in fracs {
        if new_acc % frac.den == 0 {
            new_acc = new_acc * frac.num / frac.den;
            applied_frac = Some(frac);
            break
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
    pub fn new(num: i128, den: i128) -> Frac {
        Frac { num, den }
    }

    pub fn to_rule(&self, names: &[Name]) -> Rule {
        Rule::new(
            to_names(self.den, names),
            to_names(self.num, names) 
        )
    } 
}

impl Display for Frac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{}/{}", self.num, self.den).fmt(f)
    }
}

pub fn to_names(prime_product: i128, names: &[Name]) -> Vec<Name> {
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

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

