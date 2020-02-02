use crate::{Error, Parser, ParserError, Remaining};

/// Try to consume a `str_to_match` at the beginning of the input.
pub fn label<'a>(str_to_match: &'a str) -> impl Parser<'a, &'a str> {
    move |s: Remaining<'a>| {
        if str_to_match.len() > s.rem_len() {
            return Err(Error::Failure(
                s,
                ParserError::new(
                    0..s.rem.find(|c| c == '\n').unwrap_or(s.rem.len()),
                    format!("Expected `{}` found `{}`", str_to_match, &s.rem),
                ),
            ));
        }
        let mut schars = s.rem.chars();
        let chars_to_match = str_to_match.chars();
        for i in chars_to_match.into_iter() {
            if i != schars.next().unwrap() {
                return Err(Error::Failure(
                    s,
                    ParserError::new(
                        0..s.rem.find(|c| c == '\n').unwrap_or(s.rem.len()),
                        format!(
                            "Expected `{}` found `{}`",
                            str_to_match,
                            &s.rem[..str_to_match.len() - 1]
                        ),
                    ),
                ));
            }
        }
        Ok((
            Remaining::new(&s.rem[str_to_match.len()..], s.pos + str_to_match.len()),
            str_to_match,
        ))
    }
}

/// Try to consume an unique char digit, in the given `base`
pub fn digit<'a>(base: u32) -> impl Parser<'a, char> {
    move |s: Remaining<'a>| {
        if let Some(c) = s.rem.chars().nth(0) {
            if c.is_digit(base) {
                return Ok((Remaining::new(&s.rem[c.len_utf8()..], s.pos + 1), c));
            } else {
                Err(Error::Failure(
                    s,
                    ParserError::new(0..1, format!("{} is not a digit", c)),
                ))
            }
        } else {
            Err(Error::Failure(
                s,
                ParserError::new(0..1, format!("Expected a digit, found nothing")),
            ))
        }
    }
}
/// Try to consume a double quote `"`, everything while this is not a `"`, and then the `"`. Returns only what's
/// inside the string. Doesn't support multiline string, nor escape char except escaped quote.
pub fn string<'a>() -> impl Parser<'a, &'a str> {
    // This code is clearly not elegant. Any improvements are welcome
    move |s| {
        label("\"")(s)
            .and_then(|(remaining, _)| {
                let mut last_escaped = false;
                for (k, i) in remaining.rem.chars().enumerate() {
                    if i == '\"' && !last_escaped {
                        return Ok((
                            Remaining::new(&remaining.rem[k..], remaining.pos + k),
                            &remaining.rem[..k],
                        ));
                    } else if i == '\\' {
                        last_escaped = true;
                    } else if last_escaped {
                        last_escaped = false;
                    } else if i == '\n' {
                        return Err(Error::Unsavable(
                            remaining.pos,
                            ParserError::new(0..k, "Unclosed string delimiter".to_string()),
                        ));
                    }
                }
                Err(Error::Unsavable(
                    remaining.pos,
                    ParserError::new(
                        0..remaining.rem.len(),
                        "Unclosed string delimiter".to_string(),
                    ),
                ))
            })
            .and_then(|(remaining, val)| {
                label("\"")(remaining).map(|(remaining, _)| (remaining, val))
            })
    }
}
