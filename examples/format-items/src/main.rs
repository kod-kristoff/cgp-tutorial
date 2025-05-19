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

pub trait CanFormatItems {
    fn format_items(&self) -> String;
}
impl<Context> CanFormatItems for Context
where
    for<'a> &'a Context: IntoIterator,
    for<'a> <&'a Context as IntoIterator>::IntoIter: CanFormatIter,
{
    fn format_items(&self) -> String {
        self.into_iter().format_iter()
    }
}

fn main() {
    let res = vec![1, 2, 3].format_items();
    println!("res = {res:?}");
    assert_eq!(res, "1, 2, 3");

    assert_eq!(stringly_equals(&vec![1, 2, 3], &vec![1, 2, 4]), false);
}

fn stringly_equals<Context>(left: &Context, right: &Context) -> bool
where
    Context: CanFormatItems,
{
    left.format_items() == right.format_items()
}
