mod lib;
use lib::{Memoization, Counter, square};

fn main() {
    let mut square_cacher = Memoization::new(square);
    // todo: Fix shit with double borrow
    let a = square_cacher.value(&100);
    println!("{:?}", a);

    let x = vec![5, 10, 11, 220, 32, 77, 8092];
    // x.iter() -> immutable references
    // x.iter_mut() -> mutable references
    // x.into_iter() -> objects
    for (index, item) in x.iter().enumerate() {
        println!("{index}. {item}");
    }

    let s: i32 = x.into_iter().sum(); // тут итератор потребляется
    println!("{}", s);

    let x = vec![5, 6, 7, 8, 9, 10];
    let mapped_x = x.iter().map(|i| i * i + 1);
    for i in mapped_x {
        println!("{}", i);
    }

    let c = Counter::new(6);
    for i in c {
        println!("{}", i);
    }

    println!("\n\nValues:\n\n");

    let values = Counter::new(199)
        .map(|item| item * item + 1)
        .filter(|item| item % 5 == 0);
    for (index, i) in values.enumerate() {
        println!("{index} {i}");
    }
}
