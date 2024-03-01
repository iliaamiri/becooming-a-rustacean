use std::collections::HashMap;

pub mod examples;

pub fn demo() {
    packing_order_calc();
}

pub fn more_hash_maps() {
    let mut vector = vec![3, 90, -12, 4, 0, 2, 6, 39];

    let median = examples::median(&mut vector);

    println!("{vector:?} has median of {median}");
    println!("{}", {
        let mut vector = vec![];
        let median = examples::median(&mut vector);

        format!("{vector:?} has median of {median}")
    });
    println!("{}", {
        let mut vector = vec![4];
        let median = examples::median(&mut vector);

        format!("{vector:?} has median of {median}")
    });
    println!("{}", {
        let mut vector = vec![3, 4];
        let median = examples::median(&mut vector);

        format!("{vector:?} has median of {median}")
    });
    println!("{}", {
        let mut vector = vec![3, 4, 10, 0, 2];
        let median = examples::median(&mut vector);

        format!("{vector:?} has median of {median}")
    });

    println!("---");

    println!("{}", {
        let mut string = "apple".to_string();

        let old_string = string.clone();
        format!("{} in pig latin is {}", old_string, {
            examples::to_pig_latin(&mut string)
        })
    });
    println!("{}", {
        let mut string = "first".to_string();

        let old_string = string.clone();
        format!("{} in pig latin is {}", old_string, {
            examples::to_pig_latin(&mut string)
        })
    });
    println!("{}", {
        let mut string = "".to_string();

        let old_string = string.clone();
        format!("{} in pig latin is {}", old_string, {
            examples::to_pig_latin(&mut string)
        })
    });

    examples::department_api();
}

#[allow(dead_code)]
fn last_fuckery() {
    let mut scores = HashMap::from([
                               ("Blue".to_string(), 420),
                               ("Red".into(), 69) 
    ]);

    let blue_score = scores.get(&"Blue".to_string()).copied().unwrap_or(0);

    println!("Blue scored {}", blue_score);

    let red_score = scores.get("Red").copied().unwrap_or(0);

    println!("Red scored {red_score}");

    for (key, item) in &scores {
        println!("{key}: {item}");
    }

    let _ = scores.entry("Yellow".to_string()).or_insert(0);
    let _ = scores.entry("Purple".to_string()).or_insert_with(|| blue_score * 2);
    let _ = scores.entry("Pink".to_string()).or_insert_with_key(|key| key.len());

    let teal_score = scores.entry("Teal".to_string()).or_insert(2);
    *teal_score += 2;

    scores.insert("Black".to_string(), 3);
    let _ = scores.entry("Black".to_string()).and_modify(|score| {
        if *score % 2 == 0 {
            *score = 6969
        } else {
            *score = 420420
        }
    }).or_insert(0);

    let _ = scores.entry("White".to_string()).or_insert(4);
    let white = scores.get("White").copied().unwrap_or(0);

    let removed_entry = scores.remove_entry("White");

    println!("{}", white);

    println!("{:#?}", scores);

    let (key, value) = removed_entry.unwrap_or(("Unnamed".to_string(), 0));
    println!("{key} - {value}");
}

fn packing_order_calc() -> () {
    let reusability = HashMap::<&str, u32>::from(
        [
        ("Phone charger", 0),
        ("Toothbrush", 1),
        ("Razor and Gillette", 0),
        ("socks (2 + 1 wearing)", 3),
        ("happly t-shirt", 7),
        ("long-sleeve turtle-neck.", 8),
        ("thick turtle-neck", 6),
        ("underpants (3 + 1 wearing)", 2),
        ("plastic bags", 3),
        ("surface + its charger + its bag", 5),
        ("Passport", 10),
        ("flight tickets", 10),
        ("pair of jeans", 4),
        ("pair of long-sleeve pants", 4),
        ]
        );

    let mut reusability_sorted: Vec<(&&str, &u32)> = reusability.iter().collect();
    
    reusability_sorted.sort_by(|a, b| (*a).1.cmp(&(*b).1));

    println!("Pack your items from bottom to top in this order");

    for (item, rank) in reusability_sorted.iter() {
        println!("{} --- {}", rank, item);
    }
}
