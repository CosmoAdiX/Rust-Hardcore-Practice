use core::slice;

use colored::*;

mod expressions;
mod loops;
mod ownership;
mod take_ownership;
mod referencing_dereferencing;
mod slice_type;
mod structures;
mod impl_and_associative;
mod enums;


fn main() {
    let eva: i32 = 45 + 56 + 67;
    println!("\nThe sum of some random numbers are: {eva}.");
    println!("\nHello, world!");
    print_message(10, 67);

    // calling out the expressions module here, and  explaining about the expressions
    expressions::expressions();

    let sum1 = add(4,6);
    println!("\n{} {} {} {}","So the sum of two numbers are using expression and state method are:".magenta(), sum1.to_string().green().bold(),"but this one is different because it was added directly using the operation by saving it in a variable called \"eva\": ".magenta(), eva.to_string().yellow());

    println!("\n{}", if_expression(3).to_string().red());

    //  LOOPS IN RUST
    println!("\n\n {}","======================== This is loops in rust!!! ========================".green());
    loops::loops();


    //  WHILE LOOP
    println!("\n\n {}", "======================== While Loops ========================".bold().green());
    loops::while_loops();
    println!("\n\n {}", "======================== Printing Arrays using while loop ========================".yellow());
    loops::while_using_arrays();

    
    //  FOR LOOP
    println!("\n\n {}","======================== For Loops ========================\n".bright_cyan());
    loops::for_loop();
    println!("\n\n {}","======================== Ranges in For Loop ========================".bright_blue());
    loops::ranges_in_for_loop();

    println!("\n\n {}"," ======================== Ownership ========================".green());
    ownership::ownership();

    println!("\n\n ================================ Taking Ownership in Rust ================================");
    take_ownership::take_ownership();

    let test_str1: String = String::from("Bruh wassup");

    let (s1, len) = take_ownership::calulate_len(test_str1);
    println!("The length of the string test_str1 \"{s1}\" is, {len}");

    println!("\n\n ================================ Referencing and Dereferencing ================================ ");
    referencing_dereferencing::referencing();

    println!("\n\n ================================ Slicing type in Rust ================================ ");
    slice_type::slice_text();
    
    println!("\n\n ================================ Structures in Rust ================================");
    structures::structures();

    println!("\n\n ================================ Impl And Associative Functions ================================");
    impl_and_associative::impl_function();


    println!("\n\n ================================ Enums in Rust language ================================");
    enums::enums();
    

}

fn print_message(x: i32, y: i32) {
    let z: i32 =  x + y + 999999;
    println!("\nbruh wassup nigga... \n{x} \n{y}, and the sum of them and a random big number is: {z}");
}

fn add(x: i32, y: i32) -> i32 {
    //x + y
    69
}

fn if_expression(x: i32) -> bool {
    if x % 2 == 0 {
        return true;
    }
    false    
}