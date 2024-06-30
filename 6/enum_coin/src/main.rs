#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
    Florida,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let p = value_in_cents(Coin::Penny);
    println!("{:?}", p);

    let n = value_in_cents(Coin::Nickel);
    println!("{:?}", n);

    let d = value_in_cents(Coin::Dime);
    println!("{:?}", d);

    let q = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{:?}", q);
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("ラッキー！");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}州", state);
            25
        },
    }
}
