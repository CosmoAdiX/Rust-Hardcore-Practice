use colored::*;

struct Rectangle {
    length: u16,
    width: u16,
}

pub fn structures() {
    
    println!("\n{}","|-->  Structures in Rust are like a way to group related data together, and use them throughout the program.".green());

    let new_rect = Rectangle {
        length: 11,
        width: 12,
    };

    let result_1 = rect_area(&new_rect); // here as you can see that the ownership is being transferred from the new_rect to rec and the rec to result_1 variable
    println!("|-->  The area of the rectangle is {}.", result_1.to_string().green());

    let rect2 = Rectangle {
        length: 99,
        width: 99,
    };

    let result2 = rect_area(&rect2);

    println!("The area of the rect2 is: {}.", result2);
    
}

fn rect_area(rec: &Rectangle) -> u16 {
     return rec.length * rec.width;
}

