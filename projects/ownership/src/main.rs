fn main() {
    // Variable Scope
    {                           // s is not valid here, since it hasn't been declared yet
        let mut s = "hello";  // s is valid from this point and untill the end of scope
        s = s.trim();           // you can stuff with s while we're still in the scope
        println!("{}",s);             
        // you can still do more stuff
    }                           // the scope is now over, and s is no longer valid
    // String type
    {
        let mut s = String::from("hello"); // allocates memory in the heap
        s.push_str(", world"); // appends a literal to a String
        println!("{}", s);
    }
    // Memory Allocation
    {
        let s = String::from("hello"); // allocates memory in the heap
        drop(s); // drops the s variable out of the memory forcefully, so s should not be accessible after this point 
        // println!("{}", s); // piece of code to check that the value of s is dropped as it cannot be called here due to it no longer existing
    }

    // Variables and Data interacting with Move

    {
        let x = 5;
        let y = x; // x is still valid, because it's pushed to stack due to variable being a simple value with a known size
        println!("{}", x);
        
        let s1 = String::from("hello");
        let s2 = s1; // doesnt work like that with heap objects, s1 will become invalid
        // println!("{}", s1); // get a compile time error, cause we s1 is no longer valid 
        
        // using clone to store two separate instances of data if we need to have s1 still existing after s2 assumes it's value
        let s1 = String::from("hello");
        let s2 = s1.clone(); // clone can be useful in certain scenarios, but it is memory expensive as it does a deep copy of the value in the memory
        
        println!("s1 = {}, s2 = {}", s1, s2); // no compile time error, both values are returned
    }

    {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
        // s.trim(); // this gives a compile-time error due to s being no longer valid. s.clone() can be used when passing the argument to the function to avoid runtime error.

        let mut x = 5;                      // x comes into scope

        makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
        x = x + 1;
    }
}                               

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
