fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Here we're passing the reference to s1 instead of passing s1 directly

    println!("The length of '{}' is {}.", s1, len);

    let mut another_s = String::from("hello"); // if the string is immutable it cannot be changed

    change(&mut another_s); // we need to pass a mutable reference for it to work
}

fn change(some_string: &mut String) { // and we need to accept a mutable reference as well
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize { // &String takes reference to s instread of taking the ownership of the string. So it's taking the ownership of the reference
    s.len()
}