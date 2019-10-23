use std::io::{self};

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    
    println!("Ingresa a : ");
    // unwrap
    match io::stdin().read_line(&mut input1) {
        Ok(_n) => {}
        Err(_error) => {}
    }

    println!("Ingresa b : ");
    match io::stdin().read_line(&mut input2) {
        Ok(_n) => {}
        Err(_error) => {}
    }

    println!("Ingresa c : ");
    match io::stdin().read_line(&mut input3) {
        Ok(_n) => {}
        Err(_error) => {}
    }

    let a: f32 = input1.trim().parse().expect("Wanted a number");
    let b: f32 = input2.trim().parse().expect("Wanted a number");
    let c: f32 = input3.trim().parse().expect("Wanted a number");

    let mut result: f32 = ((b * b) - 4.0 *(a * c)).abs();
    result = result.sqrt() / 2.0 * a;

    println!("El resultado es : {}", result);
}

// ownership
