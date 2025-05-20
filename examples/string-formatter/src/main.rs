use std::fmt::{self, Display};

use anyhow::Error;
use impls::{FormatAsJson, FormatStringWithDebug, FormatStringWithDisplay};

pub trait HasCgpProvider {
    type CgpProvider;
}
/// Consumer
pub trait CanFormatString {
    fn format_string(&self) -> Result<String, Error>;
}

impl<Context> CanFormatString for Context
where
    Context: HasCgpProvider,
    Context::CgpProvider: StringFormatter<Context>,
{
    fn format_string(&self) -> Result<String, Error> {
        Context::CgpProvider::format_string(self)
    }
}
/// Provider
pub trait StringFormatter<Context> {
    fn format_string(context: &Context) -> Result<String, Error>;
}
/// impls
mod impls {
    use std::fmt::{Debug, Display};

    use anyhow::Error;
    use serde::Serialize;

    use crate::StringFormatter;

    pub struct FormatStringWithDisplay;
    pub struct FormatStringWithDebug;
    impl<Context> StringFormatter<Context> for FormatStringWithDisplay
    where
        Context: Display,
    {
        fn format_string(context: &Context) -> Result<String, Error> {
            Ok(format!("{}", context))
        }
    }

    impl<Context> StringFormatter<Context> for FormatStringWithDebug
    where
        Context: Debug,
    {
        fn format_string(context: &Context) -> Result<String, Error> {
            Ok(format!("{:?}", context))
        }
    }

    pub struct FormatAsJson;

    impl<Context> StringFormatter<Context> for FormatAsJson
    where
        Context: Serialize,
    {
        fn format_string(context: &Context) -> Result<String, Error> {
            Ok(serde_json::to_string(context)?)
        }
    }
}
fn main() {
    let person = Person {
        first_name: "John".into(),
        last_name: "Smith".into(),
    };
    println!(
        "{}",
        FormatStringWithDisplay::format_string(&person).unwrap()
    );
    println!("{}", FormatStringWithDebug::format_string(&person).unwrap());
    println!("{}", FormatAsJson::format_string(&person).unwrap());
    println!("{}", person.format_string().unwrap());
}

#[derive(Debug, serde::Serialize)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

impl HasCgpProvider for Person {
    type CgpProvider = FormatStringWithDisplay;
}
