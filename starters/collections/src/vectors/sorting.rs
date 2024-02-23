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

pub fn quick_sort<T: PartialOrd>(vec: &mut Vec<T>, lo: usize, hi: usize) {
    if lo >= 0 && hi >= 0 && lo < hi {
        let p = partition(vec, lo, hi);
        quick_sort(vec, lo, p);
        quick_sort(vec, p + 1, hi);
    }
}

fn partition<T: PartialOrd>(vec: &mut Vec<T>, lo: usize, hi: usize) -> usize {
    let mut i = lo;

    let mut j = hi;

    loop { 
        while vec[i] < vec[lo] {
            i += 1;
        }
        
        while vec[j] > vec[lo] {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        vec.swap(i, j);
    }
}
