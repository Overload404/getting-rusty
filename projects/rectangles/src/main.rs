#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
} 

fn main() {
    let rect1 = Rectangle {
        width: dbg!(30),
        height: 50,
    };

    println!("rect is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}