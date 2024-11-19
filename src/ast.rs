use std::{collections::HashMap, fmt::Display};

use crate::{
    core::{self, step, to_names},
    util::get_n_primes,
};

pub type Name = String;

pub struct Prog {
    state: Vec<Name>,
    rules: Vec<Rule>,
}

impl Prog {
    // Creates a new program.
    pub fn new(state: Vec<Name>, rules: Vec<Rule>) -> Prog {
        Prog { state, rules }
    }

    // Parses and fully evaluates a program, printing each intermediate state.
    pub fn eval(&self) -> Vec<Name> {
        // get all the names
        let names = Self::get_names(&self.rules);

        // generate the "core" program: an accumulator (state) and fractions
        let mut acc = to_prime_product(&self.state, &names);
        println!(
            "{}\t\t||\t[{}]",
            acc,
            to_succinct_names(&to_names(acc, &names)).join(" ")
        );
        let fracs = self
            .rules
            .iter()
            .map(|rule| rule.to_frac(&names))
            .collect::<Vec<_>>();

        // iteratively calculate the next accumulator
        while let (new_acc, Some(frac)) = step(acc, &fracs) {
            // print the debug information
            println!(
                "{} * {}\t||\t[{}] * [{}]",
                acc,
                frac,
                to_succinct_names(&to_names(acc, &names)).join(" "),
                frac.to_rule(&names),
            );

            acc = new_acc;
        }

        // return the final accumulator
        println!(
            "{}\t\t||\t[{}]",
            acc,
            to_succinct_names(&to_names(acc, &names)).join(" ")
        );

        to_names(acc, &names)
    }

    // Gets the ordered set of names used in a list of rules, in order.
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
}

pub struct Rule {
    left: Vec<Name>,
    right: Vec<Name>,
}

impl Rule {
    // Creates a new rule.
    pub fn new(left: Vec<Name>, right: Vec<Name>) -> Rule {
        Rule { left, right }
    }

    // Extracts the names from a rule.
    fn get_names(&self) -> Vec<Name> {
        self.left.iter().chain(self.right.iter()).cloned().collect()
    }

    // Converts a rule to a fraction.
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
        )
        .fmt(f)
    }
}

// Converts a list of names to its prime product.
fn to_prime_product(vec: &[Name], names: &[Name]) -> i128 {
    let primes = get_n_primes(names.len());

    let get_prime = |name: &Name| -> i128 {
        let index = names.iter().position(|a| a.eq(name)).unwrap();
        *primes.get(index).unwrap() as i128
    };

    vec.iter().map(get_prime).fold(1, |acc, prime| acc * prime)
}

// Converts a list of names, calculating when duplicates occur
// to express them in sugared form.
// "plank log log" => "plank log^2"
// Still not canonical, since the map is not sorted.
pub fn to_succinct_names(names: &[Name]) -> Vec<String> {
    let mut map = HashMap::new();
    for name in names {
        let num: usize = match map.get(name) {
            Some(i) => *i + 1,
            None => 1,
        };

        map.insert(name, num);
    }

    map.iter()
        .map(|(a, b)| match b {
            1 => a.to_string(),
            _ => format!("{}^{}", a, b),
        })
        .collect()
}
