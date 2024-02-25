use std::{ascii::AsciiExt, ops::Index};


pub fn demo() {
    println!("Now we are learning the `String`");

    let mut full_name = String::from("Eiliy");

    full_name.push('a');

    full_name.push_str(" Abedianamiri");

    println!("Hi! My full-name is {}", full_name);

    normalize_full_name(&mut full_name);

    println!("I normalzed my full-name to: {}", full_name);

    
    println!("RTFM {1} {0}, {} {}", "yeah", "no");

    println!("Rich daddy {flex}", flex = {
        let expression = "flex box motherfucker";

        expression
    });

    let first_name = String::from("Johnathan");
    let last_name = String::from("Blow");

    let _jb_full_name = format!("{first_name} {last_name}");

    // could also get the full name like this
    let jb_full_name = first_name + " " + &last_name;

    // Flexing my dealloc skills.
    drop(last_name);

    println!("One of my favorite tech people's name is {jb_full_name}");

    let rough = String::from("A full String collection of \"heterogenous items\" aka. UTF-8 characters aka. just any characters.");

    let _rough_str = &rough[..];

    // I just dereffed the shit out of `rough` by coercing it to give me the &str. Goddammit piece of shit.
    
    let a_full = &rough[..6];

    println!("{a_full}");

    // let some_string = "آفرین";
    // this will give us run-time error!
    // let _bad_practice = &some_string[0..1];

    // good practice
    let char_indices = rough.char_indices();

    for (position, char) in char_indices {
        println!("{} {}", position, char);
    }

    let chars = rough.chars();
    for char in chars {
        print!("{}", char);
    }
    println!();

    let bytes = rough.bytes();
    for byte in bytes {
        print!("{:^3}", byte);
    }
    println!();

    println!("{rough}");
}

fn normalize_full_name(full_name: &mut String) -> &String {
    full_name.push_str(" (normalized)");

    full_name
}
