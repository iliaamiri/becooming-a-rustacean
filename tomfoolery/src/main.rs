pub mod router;

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    println!("{:#?}", four);
    println!("{:#?}", six);

    let tomFoolery = Tom::Foolery { active: true };
    let tomRod = Tom::Rod(String::from("A huge rod"));
    tomRod.talk();

    tomFoolery.talk();

    let something = Some(5);

    match something {
        None => println!("It's nothing"),
        Some(x) => println!("It's something {}", x)
    };
    let none: Option<i32> = None;

    if something.is_some_and(|x| x > 0) {
        println!("yes");
    }

    enum Test {
        One,
        Anabolic { x: i32, y: i32 },
    }

    if let None = none {
        println!("bitch");
    }

    let x = Test::Anabolic { x: 10, y: 39 };

    match &tomRod {
        Tom::Genius => {
            println!("Tom is a genius");
            tomRod
        },
        Tom::Rod(t) => {
            println!("t is: {}", t);
            tomRod
        },
        Tom::Foolery { active } => {
            println!("Tom is very active? {}", active);
            tomRod
        },
        Tom::Loom(l1, l2) => {
            println!("Loom: {:#?}", (l1, l2));
            tomRod
        }
        _ => tomRod
    };
}

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Tom {
    Genius,
    Foolery { active: bool },
    Rod(String),
    Loom(i32, i32),
    Recursive(self::IpAddrKind),
}

impl Tom {
    fn talk(&self) -> &Tom {
        println!("Calling Tom.talk: {:#?}", self);
        self
    }
}





