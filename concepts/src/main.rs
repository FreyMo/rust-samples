mod my_module {
    #[derive(Debug)]
    pub enum Day {
        Monday { test: u32, another: u64 },
        Tuesday(),
        Wednesday(String),
    }

    #[derive(Debug)]
    pub struct MyStruct {
        day: Day,
        night: Day,
        cool: Day
    }

    impl MyStruct {
        pub fn print_me(&self) -> u64 {
            println!("{:?}", self);
            5
        }

        pub fn new(day: Day, night: Day, cool: Day) -> MyStruct {
            MyStruct {
                day,
                night,
                cool
            }
        }
    }
}

use my_module::Day;
use my_module::MyStruct;

fn main() {
    let string = String::from("testy");

    let result = get_slice(&string);
    println!("{}", result);

    let test = 64;
    let another = 123;

    let my_struct = MyStruct::new(
        Day::Monday {test, another},
        Day::Tuesday(),
        Day::Wednesday(String::from(string)));

    let tup: (u32, u32, u128) = (500, 500, 10000012387455);
    let (_a, _b, _c) = tup;

    my_struct.print_me();
    println!("{}", tup.2);

    allocate_on_stack();
    allocate_on_heap();

    match maybe() {
        Some(value) => println!("{}", value),
        None => {}
    }
}

fn allocate_on_stack() {
    let _another_array = [0; 1000000];
    println!("{}", _another_array[0]);
    println!("{}", _another_array.len());
}

fn allocate_on_heap() {
    
    let mut _vec: Vec::<i32> = Vec::new();
    
    for i in 0..1000000 {
        _vec.push(i);
    }

    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(x) => println!("{}", x),
        Err(_) => println!() 
    }

    println!("You typed: {}", input.trim());
    println!("length: {}", input.trim().len());

    for _ in 0.._vec.len() {
        _vec.pop();
        if _vec.len() <= 10 {
            println!("{}", _vec.len());
        }
    }

    _vec.clear();
}

fn get_slice(string: &str) -> &str {
    let bytes = string.as_bytes();

    for (i, byte) in bytes.iter().enumerate() {
        println!("{}", i);
        println!("char: {}", byte);
    }

    &string[0..string.len()]
}

fn maybe() -> Option<u32> {
    if true {
        return Some(55);
    }

    None
}