#[path = "my_struct.rs"]
mod my_struct;
use my_struct::MyStruct;

pub fn handle_option() -> i32 {
    match cause_option(3).map(|x| x).or_else(|| Some(MyStruct {})) {
        Some(_) => return 0,
        None => return 1
    };
}

fn cause_option(input: i32) -> Option<MyStruct> {
    let _ = is_optional(input).map(|x| x)?;

    Some(MyStruct {})
}

fn is_optional(input: i32) -> Option<MyStruct> {
    match input {
        input if input >= 0 => Some(MyStruct {}),
        _ => None,
    }
}
