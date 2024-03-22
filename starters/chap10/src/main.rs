pub mod traits;

fn main() {
    traits::demo();

    let list = vec![0, 4, 60, 100, 44];

    let index = largest_value_index(&list).expect("list should have values");
    let largest_value = list[index];

    println!("largest value in the list is {}", largest_value);


    let integer_point = Point {
        x: 1, 
        y: 2,
    };

    let float_point: Point<f32> = Point {
        x: 3.4, 
        y: 2.3,
    };

    float_point.only_float32_has_this();
    float_point.x();

    println!("biaatch {:#?} {:#?}", integer_point, float_point);
}

fn largest_value_index<T: PartialOrd>(list: &Vec<T>) -> Option<usize>
{
    if list.len() < 1 {
        return None;
    }
    let mut index: usize = 0;
    let mut largest = &list[index];

    for i in 0..list.len() {
        let item = &list[i];
        if largest < item {
            largest = item;
            index = i;
        }
    }

    Some(index)
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn only_float32_has_this(&self) {
        println!("{} is a float and only this instance can see the `only_float32_has_this` method", self.x);
    }
}
