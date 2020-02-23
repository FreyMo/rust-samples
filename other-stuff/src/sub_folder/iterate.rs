#[derive(Debug)]
pub struct Foo {    
    pub x: i32,
    pub y: i32
}

impl Foo {
    fn new() -> Foo {
        Foo {
            x: 0,
            y: 0
        }
    }

    // Mutates
    fn ping(&mut self) {
        self.x = self.x + 1;
        self.y = self.y + 1;
    }
}

pub fn ping_all(foos: &mut [Foo]) {
    for foo in foos {
        foo.ping();
    }
}

pub fn ping_it() {
    let _ = do_parallel();

    let arr = vec![0; 5];

    (&arr).iter().for_each(|x| println!("{}", x));

    let mut foos = Vec::new();
    
    for _ in 0..5 {
        foos.push(Foo::new());
    }

    // If we don't borrow here it will be moved and thus no longer be available
    for foo in &foos {
        println!("{:?}", foo);
    }

    ping_all(&mut foos);

    for foo in &foos {
        println!("{:?}", foo);
    }

    while let Some(top) = foos.pop() {
        // top cannot be mutated here.
        println!("I just got popped: {:?}", top);
    }
}

extern crate rayon;

use rayon::prelude::*;
use std::time;

fn do_parallel() {
    let vec = vec![2; 500000000];

    let now = time::Instant::now();
    let sum: i32 = vec.iter().map(|x| x * x).sum();
    let elapsed_time_1 = now.elapsed().as_micros();
    // Takes about 22s in Debug, 230ms in Release
    println!("Single core:\nSum: {0}\nElapsed: {1} µs", sum, elapsed_time_1);

    let now = time::Instant::now();
    let sum: i32 = vec.par_iter().map(|x| x * x).sum();
    let elapsed_time_2 = now.elapsed().as_micros();
    // Takes about 4.8s in Debug, 84ms in Release
    println!("Single core:\nSum: {0}\nElapsed: {1} µs", sum, elapsed_time_2);

    println!("Speedup: {}", elapsed_time_1 as f64 / elapsed_time_2 as f64);
}
