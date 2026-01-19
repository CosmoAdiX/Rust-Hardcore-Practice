use std::f64::consts::PI;

struct Fruits {
    Color: String,
    Shape: String,
    radius: f64,
}

trait Shape {
    fn area_of_fruit(&self) -> f64;
}


impl Shape for Fruits {
    fn area_of_fruit(&self) -> f64 {
        return PI * self.radius * self.radius;
    }
}

pub fn traits() {
    println!("|-->    These are traits in Rust language.");
    let fruit_1 = Fruits {Color: "White".to_owned(), Shape: "Circle".to_owned(), radius: 32.2};
    let result_1 = fruit_1.area_of_fruit();
    println!("|-->    So the area of the blah-blah fruit is: {}.", result_1);

}