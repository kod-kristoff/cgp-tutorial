use std::fmt::{self, Display};

use anyhow::Error;
use impls::{FormatAsJsonString, ParseFromJsonString};

pub trait HasCgpProvider {
    type CgpProvider;
}
/// Consumers
pub trait CanFormatToString {
    fn format_to_string(&self) -> Result<String, Error>;
}

pub trait CanParseFromString: Sized {
    fn parse_from_string(raw: &str) -> Result<Self, Error>;
}

/// Provider
pub trait StringFormatter<Context> {
    fn format_to_string(context: &Context) -> Result<String, Error>;
}
pub trait StringParser<Context> {
    fn parse_from_string(raw: &str) -> Result<Context, Error>;
}

impl<Context> CanFormatToString for Context
where
    Context: HasCgpProvider,
    Context::CgpProvider: StringFormatter<Context>,
{
    fn format_to_string(&self) -> Result<String, Error> {
        Context::CgpProvider::format_to_string(self)
    }
}
impl<Context> CanParseFromString for Context
where
    Context: HasCgpProvider,
    Context::CgpProvider: StringParser<Context>,
{
    fn parse_from_string(raw: &str) -> Result<Context, Error> {
        Context::CgpProvider::parse_from_string(raw)
    }
}
/// impls
mod impls {

    use anyhow::Error;
    use serde::{Deserialize, Serialize};

    use crate::{StringFormatter, StringParser};

    pub struct FormatAsJsonString;

    impl<Context> StringFormatter<Context> for FormatAsJsonString
    where
        Context: Serialize,
    {
        fn format_to_string(context: &Context) -> Result<String, Error> {
            Ok(serde_json::to_string(context)?)
        }
    }

    pub struct ParseFromJsonString;

    impl<Context> StringParser<Context> for ParseFromJsonString
    where
        Context: for<'a> Deserialize<'a>,
    {
        fn parse_from_string(json_str: &str) -> Result<Context, Error> {
            Ok(serde_json::from_str(json_str)?)
        }
    }
}
fn main() {
    let person = Person {
        first_name: "John".into(),
        last_name: "Smith".into(),
    };
    let person_str = r#"{"first_name":"John","last_name":"Smith"}"#;
    assert_eq!(person.format_to_string().unwrap(), person_str);
    assert_eq!(Person::parse_from_string(person_str).unwrap(), person);
    println!("{}", person.format_to_string().unwrap());
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}
pub struct PersonComponents;

impl HasCgpProvider for Person {
    type CgpProvider = PersonComponents;
}

impl StringFormatter<Person> for PersonComponents {
    fn format_to_string(person: &Person) -> Result<String, Error> {
        FormatAsJsonString::format_to_string(person)
    }
}

impl StringParser<Person> for PersonComponents {
    fn parse_from_string(raw: &str) -> Result<Person, Error> {
        ParseFromJsonString::parse_from_string(raw)
    }
}
