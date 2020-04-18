use std::fmt;

#[derive(Debug)]
pub struct RangeSet {
    value: Vec<bool>,
}

impl RangeSet {
    pub fn new() -> RangeSet {
        RangeSet { value: (1..=9).into_iter().map(|_| true).collect() }
    }

    pub fn clear() -> RangeSet {
        RangeSet { value: (1..=9).into_iter().map(|_| false).collect() }
    }

    pub fn set(&self, idx: usize) -> RangeSet {
        let v = self.value
            .iter()
            .enumerate()
            .map(|(n, &b)| if n+1 == idx { true } else { b })
            .collect();
        RangeSet { value: v }
    }

    pub fn reset(&self, idx: usize) -> RangeSet {
        let v = self.value
            .iter()
            .enumerate()
            .map(|(n, &b)| if n+1 == idx { false } else { b })
            .collect();
        RangeSet { value: v }
    }

    pub fn union(&self, other: &RangeSet) -> RangeSet {
        let v = self.value.iter().zip(other.value.iter())
            .map(|(&a, &b)| if a || b { true } else { false })
            .collect();
        RangeSet { value: v }
    }

    pub fn intersection(&self, other: &RangeSet) -> RangeSet {
        let v = self.value.iter().zip(other.value.iter())
            .map(|(&a, &b)| if a && b { true } else { false })
            .collect();
        RangeSet { value: v }
    }
}

impl fmt::Display for RangeSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = self.value
            .iter()
            .enumerate()
            .filter(|(_, &b)| b)
            .map(|(n, _)| (n + 1).to_string())
            .collect::<String>();
        write!(f, "[{}]", str)
    }
}

#[test]
fn test_init() {
    let s = RangeSet::new();
    println!("{}", s);
}
