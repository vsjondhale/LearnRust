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

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50); //Specifying the width and height of the rectangle with a tuple

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect2);
    println!("rect1 is {:#?}", rect2);

    println!(
        "IMPliments The area of the rectangle is {} square pixels.",
        rect1.area()
    );

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 { //we want to borrow the struct rather than take ownership of it. 
    rectangle.width * rectangle.height
}
