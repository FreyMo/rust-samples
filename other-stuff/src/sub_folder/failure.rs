pub fn handle_failure() -> i32 {
    match cause_failure().map(|x| x * 1234) {
        Ok(_) => {
            println!("All is fine");
            return 0;
        }
        Err(_) => {
            println!("Woops!");
            return 1;
        }
    }
}

fn cause_failure() -> Result<u32, String> {
    // expect
    let _ = might_fail(31).expect("This failed horribly.alloc");

    // Refutable pattern. Dangerous here, as the failure won't be handled.
    if let Ok(i) = might_fail(22) {
        println!("{}", i);
    }

    // Standard match way
    match might_fail(30) {
        Ok(i) => println!("{}", i),
        _ => {}
    }

    // ? Operator
    // Only works on types that implement std::ops:Try
    let i = might_fail(22)?;
    println!("{}", i);
    
    Ok(0)
}

fn might_fail(input: i32) -> Result<i32, String> {
    match input {
        i if i > 30 => Ok(i + 1),
        i if i <= 0 => Ok(i),
        _ => Err(String::from("This failed miserably")),
    }
}
