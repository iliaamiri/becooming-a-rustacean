fn main() {
    println!("Welcome to my application!");

    let name = String::from("ilia");

    let user = build_a_user(name);

    println!("The user's username is: '{}'", user.username);

    let dimensions = TwoDimension(2, 10);

    let area = area(dimensions);

    println!("The area of this is: {}", area);

    fuckery();
}

fn fuckery() -> i32 {
    fn foolishnesss() {

    }

    struct Rectangle {
        width: i32,
        height: i32,
    }


    let rect = Rectangle {
        width: 32,
        height: 100
    };

    fn rectArea(rectangle: &Rectangle) -> i32 {
        rectangle.width * rectangle.height
    }

    println!("{}", rectArea(&rect));

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
