#[derive(Debug)]
enum Day {
    Monday(u32),
    Tuesday(),
}

#[derive(Debug)]
struct MyStruct {
    day: Day,
    night: Day,
}

impl MyStruct {
    fn print_me(&self) -> u64 {
        println!("{:?}", self);
        5
    }

    fn new() -> MyStruct {
        MyStruct {
            day: Day::Monday(32),
            night: Day::Tuesday()
        }
    }
}

fn main() {
    let _x = 5;
    let _x = 6; // shadowing
    let _byte: u8 = 0b11111100;

    let _ = MyStruct::new();
    let tup: (u32, u32, u128) = (500, 500, 10000012387455);
    let (_a, _b, _c) = tup;

    let my_struct = MyStruct {
        day: Day::Monday(33),
        night: Day::Tuesday(),
    };

    my_struct.print_me();
    println!("{}", tup.2);

    allocate_on_stack();
}

fn allocate_on_stack() {
    let _another_array = [0; 1000000];
    println!("{}", _another_array[0]);
    println!("{}", _another_array.len());
}
