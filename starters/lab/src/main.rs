mod fucktion;
mod restaurant;
use std::{fs::File, io::Read};

use restaurant::kitchen::prep_area;

fn main() {
    let file = File::open("test.txt");
    if file.is_err() {
        let err = file.unwrap_err();
        println!("can't open this file. {}", err);
    } else {
        let mut file = file.unwrap();
        let mut buffer = [0; 10];
        let stuff = file.read(&mut buffer[..]);

        let mut buffer = String::new();
        let stuff = file.read_to_string(&mut buffer);

        println!("content of the file: {:#?}", stuff);
    }

    println!("Welcome to my application!");

    fucktion::fucktion();
    prep_area::prepare_dish();

    let name = String::from("ilia");

    let user = build_a_user(name);

    println!("The user's username is: '{}'", user.username);

    let dimensions = TwoDimension(2, 10);

    let area = area(dimensions);

    println!("The area of this is: {}", area);

    fuckery();
}


fn fuckery() -> u32 {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.height * self.width
        }
        fn can_hold(&self, target: &Rectangle) -> bool {
            self.area() > target.area()
        }
    }

    let rect = Rectangle {
        width: 32,
        height: 100
    };

    let rect2 = Rectangle {
        width: 10,
        height: 90
    };

    println!("{}", rect.area());

    println!("rect is {:#?}", rect);

    dbg!(&rect);

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));

    rect.width
}

fn area(dimensions: TwoDimension) -> u32 {
    dimensions.0 * dimensions.1
}


struct User {
    id: i64,
    username: String,
    created_at: String,
}

fn build_a_user(name: String) -> User {
    User {
        id: 1,
        username: name,
        created_at: String::from("now"),
    }
}

struct TwoDimension(u32, u32);
