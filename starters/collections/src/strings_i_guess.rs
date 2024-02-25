
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

    let jb_full_name = first_name + " " + &last_name;

    drop(last_name);

    println!("One of my favorite tech people's name is {jb_full_name}");
}

fn normalize_full_name(full_name: &mut String) -> &String {
    full_name.push_str(" (normalized)");

    full_name
}
