fn main() {
    let number = 3;
    // else
    if number < 5 {
        println!("condition is true")
    } else {
        println!("condition is false")
    }
    // no else
    if number != 0 {
        println!("Number is not zero")
    }
    
    // else if
    let another_number = 8;
    
    if another_number % 4 == 0 {
        println!("Another number is {another_number} and it's divisible by 4")
    } else if another_number % 3 == 0 {
        println!("Another number is {another_number} and it's divisible by 3")
    } else if another_number % 2 == 0 {
        println!("Another number is {another_number} and it's divisible by 2")
    } else {
        println!("Another number is {another_number} and it's not divisible by 4, 3 or 2")
    }
    
    // if in a statement
    let condition = true;
    let yet_another_number = if condition { 5 } else { 6 };

    println!("The value of number is: {yet_another_number}");
}
