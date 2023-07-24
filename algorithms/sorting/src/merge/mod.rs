use std::fmt::Debug;
// static mut COUNT: i32 = 1;

pub fn merge_sort<T: Debug + PartialOrd + Clone>(list: &mut Vec<T>) {
    let length = list.len() - 1;
    merge_sort_range(list, 0, length)
}

pub fn merge_sort_range<T: Debug + PartialOrd + Clone>(
    list: &mut Vec<T>,
    init: usize,
    last: usize,
) {
    if last - init <= 1 {
        return;
    }
    let middle = init + (last - init) / 2;
    merge_sort_range(list, init, middle);
    merge_sort_range(list, middle, last);
    // println!("[{init}:{last}], middle: {middle}");
    merge_array(list, init, middle, last);
}

fn merge_array<T: Clone + PartialOrd + Debug>(
    list: &mut Vec<T>,
    init: usize,
    middle: usize,
    last: usize,
) {
    let left_section = list[init..middle].to_vec();
    let right_section = list[middle..last].to_vec();
    println!("[{init}:{last}], middle: {middle}");
    println!("L: {:?} - R: {:?} ", left_section, right_section,);
    println!("--------------------------------------");
    return;
    let (mut i, mut j, mut k) = (0, 0, init);

    // Check the less number in each iteration, and increment the list index
    while i < left_section.len() && j < right_section.len() {
        if left_section[i] <= right_section[j] {
            list[k] = left_section[i].clone();
            i += 1;
        } else {
            list[k] = right_section[j].clone();
            j += 1;
        }
        k += 1;
    }

    for index in i..left_section.len() {
        list[index] = left_section[i].clone();
        i += 1;
        k += 1;
    }
    for index in j..right_section.len() {
        list[index] = right_section[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod merge_sort_test {
    use super::merge_sort_range;

    #[test]
    fn first_test() {
        let mut list = [0, 2, 4, 6, 7, 1, 2, 3, 4, 5, 9, 10].to_vec();
        let (first, last) = (1, list.len() - 2);
        println!("merge: {first}, {last}");
        merge_sort_range(&mut list, first, last);
        assert_eq!(list, [0, 1, 2, 2, 3, 4, 4, 5, 6, 7, 9, 10].to_vec());
    }

    // #[test]
    fn testing() {
        let mut list = [12, 3, 7, 9, 14, 6, 11, 2].to_vec();
        let length = list.len();
        merge_sort_range(&mut list, 0, length);
        assert_eq!(list, [2, 3, 6, 7, 9, 11, 12, 14].to_vec())
    }
}
