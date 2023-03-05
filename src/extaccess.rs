use crate::{func2, func3, func4, Any, FNumber, FText, Param, Text};

pub enum DDEConversion {
    NumberLocalized,
    NumberEnUS,
}
impl Param for DDEConversion {
    type ParamType<'a> = &'a str;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            DDEConversion::NumberLocalized => "0",
            DDEConversion::NumberEnUS => "1",
        }
    }
}

///  Returns data from a DDE request.
#[inline]
pub fn dde(server: impl Text, topic: impl Text, item: impl Text) -> FNumber {
    FNumber(func3("DDE", &server, &topic, &item))
}

///  Returns data from a DDE request.
#[inline]
pub fn dde_conv(
    server: impl Text,
    topic: impl Text,
    item: impl Text,
    mode: DDEConversion,
) -> FNumber {
    FNumber(func4("DDE", &server, &topic, &item, &mode.as_param()))
}

///  Returns data from a DDE request.
#[inline]
pub fn dde_text(server: impl Text, topic: impl Text, item: impl Text) -> FNumber {
    FNumber(func4("DDE", &server, &topic, &item, &"2"))
}

/// Creation of a hyperlink involving an evaluated expression.
#[inline]
pub fn hyperlink(iri: impl Text, fun: impl Any) -> FText {
    FText(func2("HYPERLINK", &iri, &fun))
}
