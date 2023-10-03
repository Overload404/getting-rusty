use std::io;

fn main() {
    let mut f = String::new();

    println!("Enter temperature in Fahrenheit: ");

    // getting user input
    io::stdin()
    .read_line(&mut f)
    .expect("Something went wrong");
    
    let f: f64 = match f
        .trim()
        .parse(){
            Ok(num) => num,
            Err(_) => 0.0,
        };
    
    // calling the function
    let c = f_to_c(f);
    println!("The temperature in Celsius would be: {}", c);
}

// conversion function
fn f_to_c(temp: f64) -> f64 { // function return type is the important bit, the '-> f64'
    let cels:f64 = (temp - 32.0) / 1.8;
    cels
}