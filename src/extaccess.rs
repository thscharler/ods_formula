use crate::{func, Any, FNumber, FText, Text};

pub enum DDEConversion {
    NumberLocalized,
    NumberEnUS,
}
impl Any for DDEConversion {
    fn formula(&self, buf: &mut String) {
        match self {
            DDEConversion::NumberLocalized => buf.push('0'),
            DDEConversion::NumberEnUS => buf.push('1'),
        }
    }
}

pub fn dde<S: Text, T: Text, I: Text>(
    server: S,
    topic: T,
    item: I,
    mode: Option<DDEConversion>,
) -> FNumber {
    if let Some(mode) = mode {
        FNumber(func("DDE", &[&server, &topic, &item, &mode]))
    } else {
        FNumber(func("DDE", &[&server, &topic, &item]))
    }
}

pub fn dde_text<S: Text, T: Text, I: Text>(server: S, topic: T, item: I) -> FNumber {
    FNumber(func("DDE", &[&server, &topic, &item, &"2"]))
}

pub fn hyperlink<T: Text, F: Any>(iri: T, fun: F) -> FText {
    FText(func("HYPERLINK", &[&iri, &fun]))
}
