use std::fmt::{self, Display};

use impls::{FormatStringWithDebug, FormatStringWithDisplay};

/// Consumer
pub trait CanFormatString {
    fn format_string(&self) -> String;
}
/// Provider
pub trait StringFormatter<Context> {
    fn format_string(context: &Context) -> String;
}
/// impls
mod impls {
    use std::fmt::{Debug, Display};

    use crate::StringFormatter;

    pub struct FormatStringWithDisplay;
    pub struct FormatStringWithDebug;
    impl<Context> StringFormatter<Context> for FormatStringWithDisplay
    where
        Context: Display,
    {
        fn format_string(context: &Context) -> String {
            format!("{}", context)
        }
    }

    impl<Context> StringFormatter<Context> for FormatStringWithDebug
    where
        Context: Debug,
    {
        fn format_string(context: &Context) -> String {
            format!("{:?}", context)
        }
    }
}
fn main() {
    let person = Person {
        first_name: "John".into(),
        last_name: "Smith".into(),
    };
    println!("{}", FormatStringWithDisplay::format_string(&person));
    println!("{}", FormatStringWithDebug::format_string(&person));
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}
