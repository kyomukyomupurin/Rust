#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[allow(dead_code)]
struct Color(i32, i32, i32);
#[allow(dead_code)]
struct Point(i32, i32, i32);

/*
fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/

/*
#[allow(dead_code)]
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
/*
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
*/

    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
