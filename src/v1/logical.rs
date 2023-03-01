use crate::v1::func;
use crate::*;
use std::fmt::Write;

// pub struct IfFn<C, T, E> {
//     condition: C,
//     then: T,
//     otherwise: E,
// }
//
// impl<C: Logical, T: Any, E: Any> Any for IfFn<C, T, E> {
//     fn formula(&self, buf: &mut dyn Write) -> std::fmt::Result {
//         func("IF", &[&self.condition, &self.then, &self.otherwise], buf)
//     }
// }
//
// impl<C: Logical, T: Number, E: Number> Number for IfFn<C, T, E> {}
// impl<C: Logical, T: Text, E: Text> Text for IfFn<C, T, E> {}
// impl<C: Logical, T: Logical, E: Logical> Logical for IfFn<C, T, E> {}
// impl<C: Logical, T: Reference, E: Reference> Reference for IfFn<C, T, E> {}
//
// fn if_<C: Logical, T: Any, E: Any>(condition: C, then: T, otherwise: E) -> IfFn<C, T, E> {
//     IfFn {
//         condition,
//         then,
//         otherwise,
//     }
// }
