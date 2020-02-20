#[path = "my_struct.rs"]
mod my_struct;
use my_struct::MyStruct;

pub fn optional(input: i32) -> Option<MyStruct> {
    match input {
        input if input >= 0 => Some(MyStruct {}),
        _ => None,
    }
}
