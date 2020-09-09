use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
/*
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
*/

/*
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
*/

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //let mut s = String::new();
/*
    let data = "initial contents";
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 is {}", s2);
*/

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
/*
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
*/
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
/*
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
*/
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
