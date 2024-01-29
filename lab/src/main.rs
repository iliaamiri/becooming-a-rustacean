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

fn fuckery() -> u32 {
    fn foolishnesss() {

    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.height * self.width
        }
    }


    let rect = Rectangle {
        width: 32,
        height: 100
    };

    println!("{}", rect.area());

    println!("rect is {:#?}", rect);

    dbg!(&rect);

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
