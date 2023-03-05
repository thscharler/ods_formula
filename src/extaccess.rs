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

///  Returns data from a DDE request.
#[inline]
pub fn dde(
    server: impl Text,
    topic: impl Text,
    item: impl Text,
    mode: Option<DDEConversion>,
) -> FNumber {
    if let Some(mode) = mode {
        FNumber(func("DDE", &[&server, &topic, &item, &mode]))
    } else {
        FNumber(func("DDE", &[&server, &topic, &item]))
    }
}

///  Returns data from a DDE request.
#[inline]
pub fn dde_text(server: impl Text, topic: impl Text, item: impl Text) -> FNumber {
    FNumber(func("DDE", &[&server, &topic, &item, &"2"]))
}

/// Creation of a hyperlink involving an evaluated expression.
#[inline]
pub fn hyperlink(iri: impl Text, fun: impl Any) -> FText {
    FText(func("HYPERLINK", &[&iri, &fun]))
}
