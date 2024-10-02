use std::collections::HashMap;

use crate::core;

pub type Name = String;

pub struct Prog {
    state: Vec<Name>,
    rules: Vec<Rule>,
}

impl Prog {
    pub fn new(state: Vec<Name>, rules: Vec<Rule>) -> Prog {
        Prog { state, rules }
    }

    pub fn eval(&self) -> Vec<Name> {
        let names = Self::get_names(&self.rules);

        let acc = to_prime_product(&self.state, &names);
        let fracs = self.rules.iter()
            .map(|rule| rule.to_frac(&names)).collect();

        let prog = core::Prog::new(acc, fracs);

        let result = prog.eval();

        from_prime_product(result, &names)
    }

    fn get_names(rules: &[Rule]) -> Vec<Name> {
        let mut names = vec![];

        for rule in rules {
            for name in rule.get_names() {
                if !names.contains(&name) {
                    names.push(name);
                }
            }
        }

        names
    }

    pub fn print_names(names: &[Name]) -> String {
        let mut map = HashMap::new();
        for name in names {
            let val = map.get(name).unwrap_or(&0);
            map.insert(name, val + 1);
        }

        let mut string = String::new();
        for (name, val) in map {
            let new_string = if val > 1 {
                format!("{name}^{val} ") 
            } else {
                format!("{name} ")
            };
            
            string.push_str(&new_string);
        }

        string
    }
}

pub struct Rule {
    left: Vec<Name>,
    right: Vec<Name>,
}

impl Rule {
    pub fn new(left: Vec<Name>, right: Vec<Name>) -> Rule {
        Rule { left, right }
    }

    fn get_names(&self) -> Vec<Name> {
        self.left.iter().chain(self.right.iter()).cloned().collect()
    }

    fn to_frac(&self, names: &[Name]) -> core::Frac {
        core::Frac::new(
            to_prime_product(&self.right, names),
            to_prime_product(&self.left, names),
        )
    }
}

fn to_prime_product(vec: &[Name], names: &[Name]) -> i128 {
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
        
    let get_prime = |name: &Name| -> i128 { 
        let index = names.iter().position(|a| a.eq(name)).unwrap();
        *primes.get(index).unwrap() as i128
    };
    
    vec.iter()
        .map(get_prime)
        .fold(1, |acc, prime| acc * prime)
}

fn from_prime_product(prime_product: i128, names: &[Name]) -> Vec<Name> {
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

