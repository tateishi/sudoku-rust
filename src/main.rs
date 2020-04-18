use sudoku_rust::range_set::RangeSet;

fn main() {
    let a: bool = true;
    let b = RangeSet::new();
    let c = RangeSet::clear();

    println!("{}", a);
    println!("Hello, world!");
    println!("{}", b);
    let b = b.reset(3);
    println!("{}", b);

    println!("{}", c);
    let c = c.set(3);
    println!("{}", c);

    println!("{}", b.union(&c));
    println!("{}", b.intersection(&c));
}
