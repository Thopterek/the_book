#[test]
fn iterator_demo() {
    let vector = vec![1, 2];
    // iterator that takes ownership -> into_iter
    // and for mutable reference -> iter_mut
    let mut iterator = vector.iter();
    // each call to next() takes an item out
    assert_eq!(iterator.next(), Some(&1));
    // produces immutable reference
    assert_eq!(iterator.next(), Some(&2));
    // methods that call next -> consuming adapters
    assert_eq!(iterator.next(), None);
}

#[test]
fn iterator_sum() {
    let vector = vec![1, 2, 3];
    let iterator = vector.iter();
    // consuming adapter -> .sum()
    let total: i32 = iterator.sum();
    assert_eq!(total, 6);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

/*
 * fn takes ownership of vector and size param
 * into_iter to create iterator that will take ownership of passed vector
 * filter is adapting that iterator into a new one that contains only return from closure
 * closure captures through size param what to return
 * collect gathers the values returned by the adapted iterator into a vector thats returned
*/
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("weird"),
            },
            Shoe {
                size: 25,
                style: String::from("big"),
            },
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
        ];
        let models_in_size = shoes_in_size(shoes, 10);
        assert_eq!(models_in_size, vec![Shoe {size:10, style:String::from("weird")},
        Shoe {size:10, style:String::from("sneaker")},]);
    }
}
