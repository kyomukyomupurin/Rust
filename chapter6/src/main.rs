#[allow(dead_code)]
enum IpAddrKind {
    V4,
    V6,
}

#[allow(dead_code)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
fn route(ip_type: IpAddrKind) { }

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

#[allow(dead_code)]
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    /*
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    */

}
