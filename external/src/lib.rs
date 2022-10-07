pub struct Person {
    name: String,
}

impl Person {
    pub fn new(name: String) -> Self {
        Self { name: name }
    }
}
