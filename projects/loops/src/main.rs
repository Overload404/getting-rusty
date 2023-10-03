fn main() {
    // loop loop
    let mut counter = 0;

    let result = loop{
        counter += 1;
        if counter ==10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    
    // loop loop with a label
    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count +=1;
    }
    println!("End count = {count}");

    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // iterating over an array with while

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);

        index +=1;
    }

    // for loop

    for item in a {
        println!("the value is {item}");
    }

    // for loop with a range

    for number in (1..4).rev() {
        println!("{number}")
    }
    println!("LIFTOFF AGAIN!!!")
}
