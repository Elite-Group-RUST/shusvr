fn divide_numbers(a: f64, b: f64) -> Result<f64, &'static str> {
    if a.is_sign_negative() || b.is_sign_negative() {
        return Err("Usage of negative numbers is forbidden");
    }

    if b.abs() < f64::EPSILON {
        return Err("Division by zero is not allowed");
    }

    Ok(a / b)
}

fn main() {
    let x = 10.0;
    let y = 0.0;

    let result = divide_numbers(x, y);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Cannot divide {x} by {y}: {}", e),
    }

    let result = divide_numbers(x, 5.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Cannot divide {x} by {y}: {}", e),
    }
}
