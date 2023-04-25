// Define a trait with a method
trait Speak {
    fn speak(&self) -> String;
}

// Define a struct
struct Dog {
    name: String,
}

// Implement the Speak trait for the Dog struct
impl Speak for Dog {
    fn speak(&self) -> String {
        format!("Woof! My name is {}.", self.name)
    }
}

// Define an enum
enum Animal {
    Dog(Dog),
}

// Implement a method for the enum
impl Animal {
    fn get_speak(&self) -> String {
        match self {
            Animal::Dog(dog) => dog.speak(),
        }
    }
}

// Define a function that uses the enum
fn main() {
    let dog = Dog { name: String::from("Buddy") };
    let animal = Animal::Dog(dog);

    println!("The animal says: {}", animal.get_speak());
}
