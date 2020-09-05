#[allow(dead_code)]
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

#[allow(dead_code)]
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

#[allow(dead_code)]
fn gives_ownership() -> String {
    let some_string = String::from("Hello");

    some_string
}

#[allow(dead_code)]
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

/*
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
*/

#[allow(dead_code)]
fn calculate_length(s: &String) -> usize {
    s.len()
}

#[allow(dead_code)]
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

#[allow(dead_code)]
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    dbg!(slice);
}
