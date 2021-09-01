use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&(second_list.len())) {
        Ordering::Equal => {
            if compare(first_list, second_list) {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Less => {
            if slice_compare(first_list, second_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        _ => {
            if slice_compare(second_list, first_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
    }
}

fn compare<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    first_list.iter().zip(second_list).all(|(a, b)| a == b)
}

fn slice_compare<T: PartialEq>(small: &[T], large: &[T]) -> bool {
    small.len() == 0 || large.windows(small.len()).any(|win| compare(small, win))
}
