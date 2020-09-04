#[allow(dead_code)]
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

#[allow(dead_code)]
fn plus_one(x: i32) -> i32 {
    x + 1
}

#[allow(dead_code)]
fn five() -> i32 {
    5
}

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
