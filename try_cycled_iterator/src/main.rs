use cycled_iterator::CycledIterator;

fn main() {
    let items = vec!["a", "b", "c"];
    items
        .iter()
        .cycle_counted()
        .take(10)
        .for_each(|(i, c)| println!("{}: {}", i, c));
}
