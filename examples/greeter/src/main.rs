struct Person {
    name: String,
}

impl Person {
    fn name(&self) -> &str {
        &self.name
    }
}

fn greet(person: &Person) {
    println!("Hello, {}!", person.name());
}
fn main() {
    let steve = Person {
        name: "Steve".to_string(),
    };
    greet(&steve);
}
