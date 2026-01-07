use colored::*;

pub fn ownership() {
    let mut str1: &str = "Hello Brah..!!!";
    let mut x = 21;
    let y = x; // here you can see the value is copied from the variable from x to y

    x = 10;

    let str2: String = String::from("Hello"); //  THIS STRING IS STORED IN THE HEAP 

    let str3 = str2.clone();
    // str2.push_str(" Nigga!..");
    println!("==>   str2 = {str2}, str2 = {str3}");

    println!("\n==>    the string now is: {str2}");

    // so here is a function called drop() inside of String, which drops or returns the memory. it is used to destroy the operation when the value goesout of the scope.

    // Cloning 

    let str4 = str3.clone();

    // str2.push_str(", welcome in how the string can be mutated in rust...");

    println!("\n {} {}","Now it's updated into something else u see...: ".purple(), str2.yellow());

    println!("\n==>     Her eyou can see that the x is \"{x}\" and y is: \"{y}\" and the value of str4 is same as str3 which is: \"{str3}\" and \"{str4}\".");

    println!("\n================ THE CODE ENDS HERE ================");
}