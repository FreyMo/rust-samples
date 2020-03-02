use std::thread;
use std::time;

fn main() {
    let mut v = vec![0; 10];

    let result = v.iter().map(|x| x * x);

    let closure = |x: &i32| -> i32 {
        thread::sleep(std::time::Duration::from_millis(2000));
        x + 1
    };

    let _ = closure(&500);
}