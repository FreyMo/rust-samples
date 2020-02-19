
const MY_CONST: u32 = 100_000;

fn main() {
    let _x = 5;
    let _x = 6; // shadowing
    let byte: u8 = 0b11111100;

    let result = byte >> 3;

    let tup: (u32, u32, u128) = (500, 500, 10000012387455);
    let (_a, _b, _c) = tup;
    println!("{}", tup.2);

    allocate_memory_on_stack();
    println!("{}", byte);
    println!("{}", result);
    println!("{}", MY_CONST);
    println!("Hello, world!");
}

fn allocate_memory_on_stack() {
    let _another_array = [0; 1000000];
    
    println!("{}", _another_array[0]);
}