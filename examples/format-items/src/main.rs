use itertools::Itertools;
use std::fmt::Display;

fn format_iter<I>(mut iter: I) -> String
where
    I: Iterator,
    I::Item: Display,
{
    iter.join(", ")
}
fn main() {
    let res = format_iter(vec![1, 2, 3].into_iter());
    println!("res = {res:?}");
    assert_eq!(res, "1, 2, 3");
}
