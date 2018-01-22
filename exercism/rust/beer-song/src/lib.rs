use std::fmt;

struct Bottle {
    phrase: String
}

impl Bottle {
    pub fn new(n: i32, capitalize: bool) -> Bottle {
        match n {
            0 => {
                if capitalize {
                    return Bottle{phrase: String::from("No more bottles of beer")};
                } else {
                    return Bottle{phrase: String::from("no more bottles of beer")};
                }
            }
            1 => return Bottle{phrase: String::from("1 bottle of beer")},
            _ => return Bottle{phrase: format!("{} bottles of beer", n)},
        }
    }
}

impl fmt::Display for Bottle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.phrase)
    }
}

pub fn verse(n: i32) -> String {
    let b1 = Bottle::new(n, true);
    let b2 = Bottle::new(n, false);
    let mut b3 = Bottle::new(n-1, false);
    let mut advice = String::from("Take one down and pass it around");
    if n == 0 {
        advice = String::from("Go to the store and buy some more");
        b3 = Bottle::new(99, false);
    } else if n == 1 {
        advice = String::from("Take it down and pass it around");
    }
    return format!("{} on the wall, {}.\n{}, {} on the wall.\n", b1, b2, advice, b3);
}

pub fn sing(start: i32, end: i32) -> String {
    let mut out = String::new();
    for i in (end..start+1).rev() {
        out += &verse(i);
        if i != end { out += "\n"; }
    }
    out
}
