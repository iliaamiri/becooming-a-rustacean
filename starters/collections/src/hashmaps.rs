use std::collections::HashMap;

pub fn demo() {
    packing_order_calc();
}

pub fn more_hash_maps() {
    let scores = HashMap::from([
                               ("Blue".to_string(), 420),
                               ("Red".into(), 69) 
    ]);

    let blue_score = scores.get(&"Blue".to_string()).copied().unwrap_or(0);

    println!("Blue scored {}", blue_score);
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
