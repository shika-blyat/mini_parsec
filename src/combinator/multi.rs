use crate::Parser;

/// Call the predicate with the input while it returns Ok(T), and then discard the error and returns a [Vec<T>](std::vec::Vec)
pub fn many<'a, T>(mut predicate: impl Parser<'a, T>) -> impl Parser<'a, Vec<T>> {
    move |s| {
        let mut result = vec![];
        let mut remaining = s;
        while let Ok((rem, v)) = predicate(remaining) {
            result.push(v);
            remaining = rem;
        }
        Ok((remaining, result))
    }
}
/// Try to call the predicate with the input while it returns Ok(T), and then discard the error and
/// returns a [Vec<T>](std::vec::Vec). If it doesn't work atleast 1 time, the function will fail and return an Err variant
pub fn many1<'a, T>(mut predicate: impl Parser<'a, T>) -> impl Parser<'a, Vec<T>> {
    move |s| {
        let mut values = vec![];
        match predicate(s) {
            Ok((mut remaining, val)) => {
                values.push(val);
                while let Ok((rem, val)) = predicate(remaining.clone()) {
                    values.push(val);
                    remaining = rem;
                }
                Ok((remaining, values))
            }
            Err(e) => Err(e),
        }
    }
}
