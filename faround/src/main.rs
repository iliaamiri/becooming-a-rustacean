fn main() {
    let square = Rectangle::square(10);

    let rectangle = Rectangle {
        width: 5,
        height: 3
    };

    println!("{}", (3 << 1) + (5 << 1));

    dbg!("square is {}", &square);

    println!("Can square hold the rectangle? {}", square.can_hold(&rectangle));


    println!("The perimeter of the rectangle is: {}", rectangle.perimeter());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        (self.width << 1) + (self.height << 1)
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
