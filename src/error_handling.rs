use std::fs::File;

pub fn error_handling() {
    println!("\n||-->    Error handling in rust language is pretty much very important!!!\n");

    let file_1 = File::create("hello.txt");

    match File::create("bruh/hello.txt") {
        Ok(file) => println!("|->  The file inside the bruh hello.txt has been created successfully."),
        Err(error) => println!("|->  Error creating the file inside the folder bruh {:?}", error),
    }
}