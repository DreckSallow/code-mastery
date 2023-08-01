use std::fmt::Debug;

pub fn merge_sort<T: Debug + PartialOrd + Clone>(list: &mut Vec<T>) {
    let length = list.len();
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
    println!("init: {init},middle: {middle},last: {last}");
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

    while i < left_section.len() {
        list[k] = left_section[i].clone();
        i += 1;
        k += 1;
    }

    while j < right_section.len() {
        list[k] = right_section[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod merge_sort_test {
    use crate::merge::merge_sort;

    use super::merge_sort_range;

    #[test]
    fn sort_partial_array() {
        let mut list = [0, 2, 4, 6, 7, 1, 2, 3, 4, 5, 9, 10].to_vec();
        let (first, last) = (1, list.len() - 2);
        merge_sort_range(&mut list, first, last);
        assert_eq!(list, [0, 1, 2, 2, 3, 4, 4, 5, 6, 7, 9, 10].to_vec());
    }

    #[test]
    fn sort_array_using_indices() {
        let mut list = [12, 3, 7, 9, 14, 6, 11, 2].to_vec();
        let length = list.len();
        merge_sort_range(&mut list, 0, length);
        assert_eq!(list, [2, 3, 6, 7, 9, 11, 12, 14].to_vec())
    }
    #[test]
    fn sort_entire_array() {
        let list_of_arrays: [(Vec<i16>, Vec<i16>); 3] = [
            ([23, 8, 15, 4, 10].to_vec(), [4, 8, 10, 15, 23].to_vec()),
            (
                [40, 7, 21, 13, 30, 18, 5, 28, 35, 2, 11, 19, 25, 16, 32].to_vec(),
                [2, 5, 7, 11, 13, 16, 18, 19, 21, 25, 28, 30, 32, 35, 40].to_vec(),
            ),
            (
                [9, 3, 7, 1, 11, 5, 8, 2, 10, 4, 6, 12].to_vec(),
                [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec(),
            ),
        ];

        for (mut a, v) in list_of_arrays {
            merge_sort(&mut a);
            assert_eq!(a, v);
        }
    }
}
