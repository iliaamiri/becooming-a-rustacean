use std::io;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    {
        let x = "hola";
        println!("Shadowing x {x}");
    }
    let x = true;
    println!("The value of x is: {x}");

    // I'm gonna try benefitting from Shadowing in some examples.

    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("failed to read message");
    let size = size.trim();

    let size = size.len();

    println!("Size of the string is: {size}");

    // i guess this was a better example:
    let spaces = "  ";
    let spaces = spaces.len();

    println!("There are {spaces} spaces");
}
