#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn non_zero_width(&self) -> bool {
        self.width > 0    
    }

    fn contains_rectangle(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    fn square(size: u32) -> Self {
        Rectangle { length: size,
                     width: size, }
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 5,
        width: 10,
    };
    let rect2 = Rectangle {
        length: 10,
        width: 30,
    };
    println!("area: {}", rect1.area());
    println!("rect is {rect1:?}");
    println!("width being existing is {}", rect1.non_zero_width());
    println!("Can rect1 hold rect2? {}", rect1.contains_rectangle(&rect2));
}