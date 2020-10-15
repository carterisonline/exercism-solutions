#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

macro_rules! sub {
    ($list_1:expr, $list_2:expr, $replacement:expr, $result:expr) => {
        $list_1
            .into_iter()
            .enumerate()
            .filter(|(i, e)| match &$list_2.get(0) {
                Some(_) => {
                    e == &$list_1.get(*i).unwrap() && $list_1.get(*i..(i + $list_2.len())).is_some()
                }
                None => false,
            })
            .map(|(i, _e)| $list_1.get(i..(i + $list_2.len())).unwrap())
            .for_each(|x| {
                if x == $list_2 {
                    $replacement = $result;
                }
            });
    };
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    if _first_list.len() == 0 {
        return Comparison::Sublist;
    }
    if _second_list.len() == 0 {
        return Comparison::Superlist;
    }

    let mut result = Comparison::Unequal;
    sub!(_first_list, _second_list, result, Comparison::Superlist);
    sub!(_second_list, _first_list, result, Comparison::Sublist);

    result
}
