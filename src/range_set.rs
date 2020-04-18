use std::fmt;

#[derive(Debug)]
pub struct RangeSet {
    value: Vec<bool>,
}

impl RangeSet {
    pub fn new() -> RangeSet {
        RangeSet { value: (1..=9).into_iter().map(|_| true).collect() }
    }
}

impl fmt::Display for RangeSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = self.value
            .iter()
            .enumerate()
            .map(|(n, b)|
                 if *b {
                     (n+1).to_string()
                 } else {
                     String::from("")
                 })
            .collect::<String>();
        write!(f, "[{}]", str)
    }
}

#[test]
fn test_init() {
    let s = RangeSet::new();
    println!("{}", s);
}
