#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 15
    };
    let area = rect.area();
    let square = Rectangle::square(10);
    println!("{:?}", square);
    println!("{rect:#?} {area}");
}
