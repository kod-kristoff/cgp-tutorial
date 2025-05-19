pub trait HasName {
    fn name(&self) -> &str;
}
pub trait CanGreet {
    fn greet(&self);
}
impl<Context> CanGreet for Context
where
    Context: HasName,
{
    fn greet(&self)
    where
        Context: HasName,
    {
        println!("Hello, {}!", self.name());
    }
}

fn main() {
    let steve = Person {
        name: "Steve".to_string(),
    };
    steve.greet();
}
struct Person {
    name: String,
}

impl HasName for Person {
    fn name(&self) -> &str {
        &self.name
    }
}
