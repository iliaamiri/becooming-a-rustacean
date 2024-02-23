use std::vec;

pub mod sorting;

pub fn demo() {
    let mut animals: Vec<String> = Vec::new();

    animals.push("bear".to_string());

    println!("{:#?}", animals);

    let _prices = Vec::<f32>::new();

    let mut prices = vec![1, 34, 5];

    for price in &prices {
        println!("{} is a movie in the movies vector", price);
        let a: *const i32 = &*price;
        println!("Raw pointer of {} is {:#?}", price, a);
    }

    println!("prices vector looks like this: {:#?}", prices);

    sorting::bubble_sort(&mut prices);

    println!("sorted prices: {:#?}", prices);


    let mut sortable = vec![40, 20, 9, 1, 0];

    let sortable_len = sortable.len();

    sorting::quick_sort(&mut sortable, 0, sortable_len - 1);

    println!("sortable is sorted by quick_sort: {:#?}", sortable);
}

