use itertools::Itertools;
use std::fmt::Display;

fn format_iter<I>(mut iter: I) -> String
where
    I: Iterator,
    I::Item: Display,
{
    iter.join(", ")
}

fn format_items<C>(items: C) -> String
where
    C: IntoIterator,
    C::IntoIter: Itertools,
    <C::IntoIter as Iterator>::Item: Display,
{
    format_iter(items.into_iter())
}
fn main() {
    let res = format_items(&vec![1, 2, 3]);
    println!("res = {res:?}");
    assert_eq!(res, "1, 2, 3");
}
