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
        Ordering::Equal => Comparison::equal(compare(first_list, second_list)),
        Ordering::Less => Comparison::sublist(slice_compare(first_list, second_list)),
        _ => Comparison::superlist(slice_compare(second_list, first_list)),
    }
}

fn compare<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    first_list.iter().zip(second_list).all(|(a, b)| a == b)
}

fn slice_compare<T: PartialEq>(small: &[T], large: &[T]) -> bool {
    small.len() == 0 || large.windows(small.len()).any(|win| compare(small, win))
}

impl Comparison {
    fn compare(validation: bool, true_value: Comparison) -> Comparison {
        if validation {
            true_value
        } else {
            Comparison::Unequal
        }
    }
    pub fn equal(validation: bool) -> Comparison {
        Comparison::compare(validation, Comparison::Equal)
    }
    pub fn sublist(validation: bool) -> Comparison {
        Comparison::compare(validation, Comparison::Sublist)
    }
    pub fn superlist(validation: bool) -> Comparison {
        Comparison::compare(validation, Comparison::Superlist)
    }
}
