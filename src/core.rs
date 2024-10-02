#[derive(Debug)]
pub struct Prog {
    acc: i128,
    fracs: Vec<Frac>
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
}

#[derive(Debug)]
pub struct Frac {
    num: i128,
    den: i128,
}

impl Frac {
    pub fn new(num: i128, den: i128) -> Frac {
        Frac { num, den }
    }
}