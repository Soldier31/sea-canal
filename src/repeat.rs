// Ugh, this mix of imperative and functional programming feels really wrong
pub fn is_repeating_with_predicate<T, P>(slice: &[T], is_match: P) -> bool where P: Fn(&T, &T) -> bool {
    for i in 2..slice.len() {
        if slice.len() % i != 0 {
            continue;
        }

        let mut chunks = slice.chunks(i);

        let first = match chunks.next() {
            Some(chunk) => chunk,
            None => continue,
        };

        // println!("first: {:#?}", first);

        let mut matches = true;

        for chunk in chunks {
            if !chunk.iter().zip(first.iter()).into_iter().all(|(x, y)| is_match(x, y)) {
                matches = false;
                // println!("chunk: {:#?}", chunk);
                break;
            }
        }

        if matches {
            return true;
        }
    }

    false
}

#[allow(dead_code)]
pub fn is_repeating<T>(slice: &[T]) -> bool where T: Eq{
    is_repeating_with_predicate(slice, |t1, t2| t1 == t2)
}
