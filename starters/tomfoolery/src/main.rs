mod router; 

fn main() {
    println!("Hello, world!");

    router::hello_from_router();

    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    println!("{:#?}", four);
    println!("{:#?}", six);

    let tom_foolery = Tom::Foolery { active: true };
    let tom_rod = Tom::Rod(String::from("A huge rod"));
    tom_rod.talk();

    tom_foolery.talk();

    let something = Some(5);

    match something {
        None => println!("It's nothing"),
        Some(x) => println!("It's something {}", x)
    };
    let none: Option<i32> = None;

    if something.is_some_and(|x| x > 0) {
        println!("yes");
    }

    if let None = none {
        println!("bitch");
    }

    let _x = Test::Anabolic { x: 10, y: 39 };

    match &tom_rod {
        Tom::Genius => {
            println!("Tom is a genius");
            tom_rod
        },
        Tom::Rod(t) => {
            println!("t is: {}", t);
            tom_rod
        },
        Tom::Foolery { active } => {
            println!("Tom is very active? {}", active);
            tom_rod
        },
        Tom::Loom(l1, l2) => {
            println!("Loom: {:#?}", (l1, l2));
            tom_rod
        }
        _ => tom_rod
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

#[warn(dead_code)]
enum Test {
    Anabolic { x: i32, y: i32 },
}
