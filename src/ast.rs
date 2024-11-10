use std::{collections::HashMap, fmt::Display};

use crate::core::{self, step, to_names, Frac};

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

        to_names(result, &names)
    }

    pub fn step_eval(&self) -> Vec<Name> {
        let names = Self::get_names(&self.rules);

        let mut acc = to_prime_product(&self.state, &names);
        let fracs = self.rules.iter()
            .map(|rule| rule.to_frac(&names)).collect::<Vec<_>>();

        while let (new_acc, Some(frac)) = step(acc, &fracs) {
            let state = to_names(new_acc, &names);
            let rule = frac.to_rule(&names);

            println!(
                "{} [{}] * {} [{}] => {} [{}]",
                acc, to_succinct_names(&to_names(acc, &names)).join(" "),
                frac, frac.to_rule(&names),
                new_acc, to_succinct_names(&to_names(new_acc, &names)).join(" ")
            );

            acc = new_acc;
        }

        to_names(acc, &names)
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

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(
            "{} -> {}", 
            to_succinct_names(&self.left).join(" "), 
            to_succinct_names(&self.right).join(" ")
        ).fmt(f)
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

fn to_succinct_names(names: &[Name]) -> Vec<String> {
    let mut map = HashMap::new();
    for name in names {
        let num: usize = match map.get(name) {
            Some(i) => *i + 1,
            None => 1,
        };

        map.insert(name, num);
    }

    map.iter().map(|(a,b)| match b {
        1 => a.to_string(),
        _ => format!("{}^{}", a, b)
    }).collect()
}