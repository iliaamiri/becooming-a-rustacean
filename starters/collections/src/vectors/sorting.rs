pub fn bubble_sort<T: PartialOrd>(vec: &mut[T]) {
    if vec.len() <= 1 {
        return;
    }

    let mut swaped;

    loop {
        swaped = false;

        for i in 0..vec.len() - 1 {
            if vec[i + 1] < vec[i] {
                vec.swap(i, i + 1);
                swaped = true;
            }
        }

        if !swaped {
            break;
        }
    }
}

pub fn quick_sort() {}
