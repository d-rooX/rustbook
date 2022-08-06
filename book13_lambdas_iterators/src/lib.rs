use std::collections::HashMap;
use std::hash::Hash;

/// Делает какое то движение
///
/// # Примеры
///
/// ```
/// let a = square(&13);
/// println("{}", a); // -> 13 * 13;
/// ```
pub fn square(arg: &i32) -> Vec<i32> {
    let mut vec = vec![];
    for i in 0..*arg {
        vec.push(i);
    }
    vec
}

pub struct Memoization<'a, F, A, V>
    where F: Fn(&'a A) -> V,
          A: Hash + Eq
{
    calculate: F,
    values: HashMap<&'a A, V>
}

impl<'a, F, A, V> Memoization<'a, F, A, V>
    where F: Fn(&'a A) -> V,
          A: Hash + Eq
{
    pub fn new(func: F) -> Self {
        Self {
            calculate: func,
            values: HashMap::new()
        }
    }
    pub fn value(&'a mut self, arg: &'a A) -> &'a V {
        if self.values.contains_key(arg) {
            self.values.get(arg).unwrap()
        } else {
            let res = (self.calculate)(arg);
            self.values.insert(arg, res);
            self.values.get(arg).unwrap()
        }
    }
}


pub struct Counter {
    count: u32,
    to: u32
}
impl Counter {
    pub fn new(to: u32) -> Counter {
        Counter { count: 0, to }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < self.to {
            Some(self.count)
        } else {
            None
        }
    }
}