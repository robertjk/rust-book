#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2: Rectangle = Rectangle::square(25);
    let rect3 = Rectangle {
        width: 70,
        height: 45,
    };

    println!(
        "The area of the rectangle 1 is {} square pixels.",
        rect1.area()
    );
    println!(
        "The area of the rectangle 2 is {} square pixels.",
        rect2.area()
    );
    println!(
        "The area of the rectangle 3 is {} square pixels.",
        rect3.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
