use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    // loops();
    // strings();
    // ownership();
    let string = String::from("Hello, world!");
    let first = first_word(&string);
    println!("{}\n{}", string, first);

    let first = another_first_word("hello world");
    println!("{}", first);
}

fn ownership() {
    let mut x = String::from("hello");
    x = takes_and_gives_back(x);
    println!("{}", x);
}

fn another_first_word(s: &str) -> &str {
    for (index, char) in s.chars().enumerate() {
        if char == ' ' {
            return &s[..index];
        }
    }
    &s[..]
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn takes_and_gives_back(mut x: String) -> String {
    x.push_str("ahaha");
    x
}


fn loops() {
    let mut res = calc(1, 2);

    let looped = {
        let mut counter = 0;
        loop {
            counter += 1;
            if counter == 100 {
                break counter
            }
        }
    };

    while res != 0 {
        print!("HA");
        res -= 1;
    }
    println!("{}", looped);


    let numbers: [u32; 5] = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        println!("While {}", numbers[index]);
        index += 1;
    }

    for element in numbers.iter() {
        println!("For {}", element);
    }

    for num in (0..10).rev() {
        println!("Reversed {}", num);
    }
}

fn strings() {
    let mut str = String::from("hello");
    str.push_str(", world!");
    let mut str1 = str;

    str1.push_str(" Hello again!");
    println!("{}", str1);

    print(&mut str1);
    println!("{}", str1);
}

fn print(x: &mut String) {
    println!("Printed {}", x);
    x.push_str("AHAHH CHANGED");
}

fn calc(x: u32, y: u32) -> u32 { x + y }
