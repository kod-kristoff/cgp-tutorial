trait HasName {
    fn name(&self) -> &str;
}
fn greet<Context>(context: &Context)
where
    Context: HasName,
{
    println!("Hello, {}!", context.name());
}

fn main() {
    let steve = Person {
        name: "Steve".to_string(),
    };
    greet(&steve);
}
struct Person {
    name: String,
}

impl HasName for Person {
    fn name(&self) -> &str {
        &self.name
    }
}
