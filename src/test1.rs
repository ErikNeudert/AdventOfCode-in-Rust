//testing dependency inversion principle, e.g.
//specifying the most general type you want to use
//via Boxing: Box<dyn Trait>
fn main() {
    forward(3);
    println!("");
    backward(3);
}

fn forward(num: i32) {
    fn passthrough(iterator: Box<dyn DoubleEndedIterator<Item=i32>>) -> Box<dyn DoubleEndedIterator<Item=i32>> {
        iterator
    }
    iterate(num, passthrough);
}
fn backward(num: i32) {
    fn invert(iterator: Box<dyn DoubleEndedIterator<Item=i32>>) -> Box<dyn DoubleEndedIterator<Item=i32>> {
        Box::from(iterator.rev())
    }
    iterate(num, invert);
}
fn iterate(num: i32, iterator_modifier: fn(Box<dyn DoubleEndedIterator<Item=i32>>) -> Box<dyn DoubleEndedIterator<Item=i32>>) {
    let mut range = 0..num;
    let mut range = iterator_modifier(Box::from(range));
    for i in range {
        print!("{}", i);
    }
}