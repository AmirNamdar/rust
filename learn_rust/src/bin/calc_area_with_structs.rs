struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area of the rectangle is {}", area(rect));
}
