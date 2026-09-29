fn main() {
    println!("Hello, world!");

    another_function(3, "cm");
}

fn another_function(x: i32, unit: &str) {
    println!("The value of x is {x} {unit}")
}