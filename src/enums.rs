use std::any::Any;



#[derive(Debug)]
enum Color {
    //These are variants
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Debug)]
enum Shape {
    Square,
    Circle,
}

impl Shape {
    fn new_circle() -> Self {
        Self::Circle
    }

    fn new_square() -> Self {
        Self::Square
    }
}

pub fn enums() {
    println!("|-->    Enums in Rust language....");
    println!("So there are basically three types of enums ----->");
    println!(r#"These are 
    |--> Simple Enums
    |--> Optional Enums
    |--> Result Enums"#);

    let color: Color = Color::Red;
    // let type_of = color.into();

    println!("|-->   This is a {:?}", color);


    // So this is simple ENUMS
    match color{
        Color::Red=>println!("|-->    It's red!"),
        Color::Yellow=>println!("|-->    It's Yellow!"),
        Color::Green=>println!("|-->    It's Green!"),
        Color::Blue=>println!("|-->    It's Blue!"),
    }

    let circle_1 = Shape::Circle; // So this is the old way, but we can also use the implement method , as it is very modern and fast

    let circle_2 = Shape::new_circle();
    println!("|-->    This is the second circle you were talking about {:?}", circle_2);

    // So i have imlemented the method for the Square as well...
    let square_1 = Shape::new_square();
    println!("|-->    So this is the first square i was talking about, {:?}", square_1);


    
}   