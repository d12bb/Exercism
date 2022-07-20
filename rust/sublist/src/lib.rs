#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    use Comparison::*;

    if first.len() == second.len() && first == second {
        return Equal;
    }
    if first.len() == 0 {
        return Sublist;
    }
    if first.len() > second.len() && sublist(second, first) == Sublist {
        return Superlist;
    }

    for win in second.windows(first.len()) {
        if win == first {
            return Sublist;
        }
    }

    return Unequal;
}
