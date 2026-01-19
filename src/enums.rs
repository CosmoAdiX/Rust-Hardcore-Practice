use std::{any::Any, f64::consts::PI};
use std::fmt::Display;


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
    Square(f64, f64), // so now we will add parameters or the value to the Shape, many people will confuse it with structures but actually it's different from each other, actually a lot.
    Circle(f64),
}

// Optional Enums
#[derive(Debug)]
enum OptionalValue<T> {
    Some(T),
    None,
}

// RESULT ENUM
// #[derive(Debug)]
// enum ResultEnum



impl Shape {
    fn new_circle(radius: f64) -> Self {
        /* Remember that when you use the structures we used something like this
            struct Triangle {
                base: u8,
                height: u8,
            }

        this is how you make a struct but instead of that in enums you're basically just making the options of a datatype, but you can still
        perform calculations in enums and use it with "impl".

        -> Also while definn=ing the enums, when giving them value you use "()" not ":" like defining it in struct ex:
            enum Animals {
                Lion("here you give jus the data type and nothing else."),
                Cheeta(f64/i)
            }
         */
        Self::Circle(radius)
    }


    // ------------------------------  METHODS ----------------------------------------
    fn new_square(width: f64, heigth: f64) -> Self {
        Self::Square(width, heigth)
    }

    fn circle_area(radius: f64) -> f64 {
        PI * radius * radius
    }

    // Hence there is another method also very wise method, using match inside a custom function
    fn area(&self) {
        match self {
            Shape::Circle(radius) => println!("|-->    So the are of the circle is, {:?}", (PI * radius * radius)),
            Shape::Square(width,height) => println!("|-->    So the area of the rectangle is, {:?}.", width * height),
        }
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

    let circle_1 = Shape::Circle(12.1); // So this is the old way, but we can also use the implement method , as it is very modern and fast

    let circle_2 = Shape::new_circle(8.0);
    println!("|-->    This is the second circle you were talking about {:?}", circle_2);

    // So i have imlemented the method for the Square as well...
    let square_1 = Shape::new_square(10.0,10.0);
    println!("|-->    So this is the first square i was talking about, {:?}", square_1);

    let area_1 = Shape::circle_area(1.0);
    println!("|-->    So the area of the circle is: {:?} square unit.", area_1);

    square_1.area();
    circle_2.area();

    let user_id_1 = 1;
    let user_id_2 = 22;
    

    let u0 = get_user_phonenumber(user_id_1);
    let u1 = get_user_phonenumber(user_id_2);

    println!("|-->    Bruh it's: {:?}", u0);
    println!("|-->    Bruh it's: {:?}", u1);

    let divide_1 = divide(12, 12);
    println!("|-->    The division is: {:?}", divide_1);

    // So here this is the generic type of enum, where you basically return any type of value using <T: Display>.
    let string_1 = String::from("Bruh wassup...");
    print_stupid_data(string_1);

    // To check even more you can put many other kinds of data types, like:-->
    let x1 = 22_f64/7_f64;
    let x2 = true;
    let x3 = "hello nigga".to_owned();

    print_stupid_data(x1);
    print_stupid_data(x2);
    print_stupid_data(x3);

}   

fn get_user_phonenumber(user_id: i32) -> Option<i32> {
    let mob_num = 9445;
    if user_id == 1 {
        return Some(mob_num);
    } else {
        // println!("|-->    stfu, you're not the sigma..!!!");
        return None;
    }
}

fn divide(x: i32, y: i32) -> Result<i32,String> {
    if y == 0 {
        return Err("Y is zero".to_owned());
    } else {
        return Ok(x/y);
    }
}

// #[derive(Debug)]
fn print_stupid_data<T: Display>(data: T) {
    println!("|-->    Bruh the data is: {}.", data);
}