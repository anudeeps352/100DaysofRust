// Traits are very similiar to Interfaces in other languages. Traits are behaviour that other types must implement

//trait Describable

trait Describable {
    // A trait method that must be implemented by any type that uses this trait.
    fn describe(&self)->String;
    // we can also define a function body which will be later overriden as well if not implemented
}

struct Person{
    name:String,
    age: u32,
}

struct Car{
    brand:String,
    model:String,
}

impl Describable for Person {
    fn describe(&self)-> String {
        format!("Person: {} is {} years old",self.name,self.age)
    }
}

impl Describable for Car {
    fn describe(&self)-> String {
        format!("Car: {}  {} ",self.brand,self.model)
    }
}


// A function that accepts any type that implements the `Describable` trait

// <T:Describable> is a trait bound
fn print_description<T: Describable>(item: &T) {
    println!("{}", item.describe());
}

fn main() {
    let person = Person{
        name: String::from("Alice"),
        age: 23,
    };

    let car = Car {
        brand: "Toyota".to_string(),
        model: "Corolla".to_string(),
    };
     // Call `describe()` method directly
    println!("{}", person.describe());
    println!("{}", car.describe());

    // Call the function that accepts anything implementing `Describable`
    print_description(&person);
    print_description(&car);
}
