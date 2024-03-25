/*fn area(width: u32, height: u32) -> u32 {
    width * height
}*/

/*fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    //let width1 = 30;
    //let height1 = 50;
    //let rect1 = (30, 50);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    println!("Rectangle data, width={}, height={}", rect1.width, rect1.height);

    println!("Rectangle = {:?}", rect1);
    println!("Rectangle = {:#?}", rect1);

    dbg!("Rectangle = {}", rect1);
    // To pretty-print the struct we can use: {:#?}
    //
    println!("---------------------------- ....... ---------------------------");

    let scale = 2;

    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}
