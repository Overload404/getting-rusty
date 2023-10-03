use std::cmp::Ordering;

fn main() {
    print!("Functions ");
    // calling another function
    another_function();

    let number1:i32 = 10;
    let number2:i32 = 20;
    let number3:i32 = 30;
    // passing variables as arguments
    let comparison_array: [&str; 3] = ["greater than", "less than", "equal to"];
    multiply_by_5(number1, comparison_array);
    multiply_by_5(number2, comparison_array);
    multiply_by_5(number3, comparison_array);
    print_labeled_measurement(5, 'h');

    // assigning value from function
    let five_function_value = five();
    // value is returned 
    println!("The value is {five_function_value}");
    // another example of the above
    let number = plus_one(5);
    println!("The value of number is: {number}");
}

// just another function
fn another_function() {
    println!("can call other functions");
}

// accepting a parameter and working through it. datatype must be specified
fn multiply_by_5(number: i32, comparison_array: [&str; 3]) {
    let outcome: i32 = number * 5;
    let hundred: i32 = 100;
    let comparison_result: &str = match &outcome.cmp(&hundred) { 
        Ordering::Greater => comparison_array[0],
        Ordering::Less => comparison_array[1],
        Ordering::Equal => comparison_array[2]
    };
    println!("{number} multiplied by 5 is {outcome}. Which is {comparison_result} {hundred}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// function with a return value

fn five() -> i32 {
    5
}

fn plus_one(number: i32) -> i32 {
    number + 1
}