use crate::{Parser, Remaining};

/// Consume everything char by char until it encounters something which is not a whitespace
pub fn ws<'a>() -> impl Parser<'a, ()> {
    |s: Remaining<'a>| match s.rem.find(|c: char| !c.is_whitespace()) {
        Some(index) => Ok((Remaining::new(&s.rem[index..], s.pos + index), ())),
        None => Ok((
            Remaining::new(&s.rem[s.rem.len()..s.rem.len()], s.pos + s.rem.len()),
            (),
        )),
    }
}
