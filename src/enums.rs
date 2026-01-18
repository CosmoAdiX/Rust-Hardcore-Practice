use std::{any::Any, f64::consts::PI};



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
    

    get_user_phonenumber(user_id_1);
    get_user_phonenumber(user_id_2);
}   

fn get_user_phonenumber(user_id: i32) {
    if user_id == 1 {
        println!("shut the fuck up bro...!");
    } else {
        println!("stfu");
    }
}