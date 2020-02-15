
macro_rules! my_first_macro {
    () => {
        println!("Hi there!");
    };
}

fn main() {
    let _vector: Vec<u32> = Vec::new();

    let array = vec![1, 2, 3, 4];

    match array.len() {
        1 => println!("1"),
        _ => println!("anything"),
    }

    my_first_macro!();
    println!("{:?}", array);
}

struct MyGuy {
    name: String,
    age: u32
}

trait CanBePrinted {
    fn print_me(&self);
}

impl CanBePrinted for MyGuy {
    fn print_me(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
    }
}