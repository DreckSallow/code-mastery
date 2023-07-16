pub fn sort_with_insertion<T>(list: &mut Vec<T>)
where
    T: PartialOrd + Copy,
{
    if list.len() < 2 {
        return;
    }
    for i in 1..list.len() {
        let current_val = list[i];

        let mut j = (i as i32) - 1;

        while j >= 0 && (list[j as usize] > current_val) {
            list[(j + 1) as usize] = list[j as usize];
            j -= 1;
        }
        list[(j + 1) as usize] = current_val;
    }
}

#[cfg(test)]
mod test {
    use super::sort_with_insertion;

    #[test]
    fn allow_less_length() {
        let mut less_list = vec![9];
        sort_with_insertion(&mut less_list);
        assert_eq!(less_list, vec![9]);
    }

    #[test]
    fn test_numbers() {
        let mut list_to_sort = vec![8, 4, 5, 1, 6, 0, 7];
        sort_with_insertion(&mut list_to_sort);
        assert_eq!(list_to_sort, vec![0, 1, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn worst_case() {
        let mut list_to_sort = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        sort_with_insertion(&mut list_to_sort);
        assert_eq!(list_to_sort, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
