
fn main() {
        
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// let x = 5;
// println!("The value of x is: {x}");
// {
//     let x = "hola";
//     println!("Shadowing x {x}");
// }
// let x = true;
// println!("The value of x is: {x}");

// // I'm gonna try benefitting from Shadowing in some examples.

// let mut size = String::new();

// let s: &str = "hloooo   \n ";

// let _ss = s.trim();
// let _sssss = &mut size;

// println!("{s}");

// io::stdin()
//     .read_line(&mut size)
//     .expect("failed to read message");
// let size = size.trim();

// let size = size.len();

// println!("Size of the string is: {size}");

// // i guess this was a better example:
// let spaces = "  ";
// let spaces = spaces.len();

// println!("There are {spaces} spaces");
// }
