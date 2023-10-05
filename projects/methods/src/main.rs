#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn zero_width(&self) -> bool {
        self.width == 0
    }
}

impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool{
        self.width > rect.width && self.height > rect.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };


    if rect1.zero_width() {
        println!("The rectangle has 0 width")
    }
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let rect4 = Rectangle {
        width: 20,
        height: 65,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));

}
