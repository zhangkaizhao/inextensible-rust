use external::Person;

pub trait PersonExt {
    fn greet(&self);
}

impl PersonExt for Person {
    fn greet(&self) {
        // error[E0616]: field `name` of struct `Person` is private
        println!("Hello, {}!", self.name);
    }
}


fn main() {
    let ex = Person::new("Rust".to_string());
    ex.greet();
}
