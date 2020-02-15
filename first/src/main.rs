
macro_rules! my_first_macro {
    () => {
        println!("Hi there!");
    };
}

fn main() {
    let _vector: Vec<u32> = Vec::new();

    let array = vec![1, 2, 3, 4];

    my_first_macro!();
    println!("{:?}", array);
}
