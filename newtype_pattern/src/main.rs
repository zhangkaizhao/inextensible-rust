use external::Person;

pub struct WrappedPerson(Person);

impl WrappedPerson {
    fn greet(&self) {
        // error[E0616]: field `name` of struct `Person` is private
        println!("Hello, {}!", self.0.name);
    }
}


fn main() {
    let person = Person::new("Rust".to_string());
    let wrapped = WrappedPerson(person);
    wrapped.greet();
}
