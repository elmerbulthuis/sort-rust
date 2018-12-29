use std::cmp::Ordering;
use std::collections::HashMap;

fn item_compare(index: &HashMap<String, i32>, a: &String, b: &String) -> Ordering {
    let ia = index.get(a).unwrap();
    let ib = index.get(b).unwrap();
    return ia.cmp(ib);
}

#[allow(dead_code)]
fn make_item_comparer<'a>(
    index: &'a HashMap<String, i32>,
) -> impl FnMut(&'a String, &'a String) -> Ordering {
    return move |a, b| item_compare(&index, a, b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut index = HashMap::new();
        index.insert("c".to_string(), 3);
        index.insert("a".to_string(), 2);
        index.insert("b".to_string(), 1);

        let mut list = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        // this works!
        list.sort_by(|a, b| item_compare(&index, a, b));

        // how to make this work?
        // list.sort_by(make_item_comparer(&index));

        assert_eq!(
            list,
            vec!["b".to_string(), "a".to_string(), "c".to_string()]
        )
    }
}