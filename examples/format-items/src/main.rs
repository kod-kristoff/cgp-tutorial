use itertools::Itertools;
use std::fmt::Display;

pub trait CanFormatIter {
    fn format_iter(self) -> String;
}

impl<I> CanFormatIter for I
where
    I: Itertools,
    I::Item: Display,
{
    fn format_iter(mut self) -> String {
        self.join(", ")
    }
}

fn format_items<C>(items: C) -> String
where
    C: IntoIterator,
    C::IntoIter: CanFormatIter,
{
    items.into_iter().format_iter()
}
fn main() {
    let res = format_items(&vec![1, 2, 3]);
    println!("res = {res:?}");
    assert_eq!(res, "1, 2, 3");
}
