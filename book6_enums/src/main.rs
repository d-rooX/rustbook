enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum State {
    Minnesota,
    Man,
    Ohio,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from state {:?}", state);
            25
        }
    }
}

fn plus(var: Option<i32>, x: i32) -> Option<i32> {
    match var {
        Some(i) => Some(i + x),
        None => None
    }
}

fn main() {
    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let coin = Coin::Quarter(State::Ohio);
    let value = value_in_cents(&coin);
    println!("{value}");

    let num = None;
    println!("{:?}", plus(num, 10));
    let num = Some(10);
    println!("{:?}", plus(num, 10));


    let mut count = 0;
    let coins = [
        Coin::Dime,
        Coin::Quarter(State::Man),
        Coin::Nickel,
        Coin::Quarter(State::Minnesota)
    ];

    for coin in coins {
        if let Coin::Quarter(state) = coin {
            println!("Четвертак из штата {state:?}");
            count += 1;
        } else {
            count += 1;
        }
    };
    println!("{count}");
}













