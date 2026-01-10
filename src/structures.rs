use colored::*;

struct Rectangle {
    length: u8,
    width: u8,
}

pub fn structures() {
    
    println!("\n{}","|-->  Structures in Rust are like a way to group related data together, and use them throughout the program.".green());

    let new_rect = Rectangle {
        length: 11,
        width: 12,
    };

    let result_1 = rect_area(&new_rect); // here as you can see that the ownership is being transferred from the new_rect to rec and the rec to result_1 variable
    println!("|-->  The area of the rectangle is {}.", result_1.to_string().green());
    
}

fn rect_area(rec: &Rectangle) -> u8 {
     return rec.length * rec.width;
}

