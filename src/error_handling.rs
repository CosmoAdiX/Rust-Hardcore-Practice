use std::fs::File;
use std::io;

pub fn error_handling() {
    println!("\n||-->    Error handling in rust language is pretty much very important!!!\n");

    // let file_1 = File::create("invalid/jajajajsa\0hello.txt");

    // match File::create("invalid/jajajajsa\0hello.txt") {
    //     Ok(file) => println!("|->  The file inside the bruh hello.txt has been created successfully. so the file is {:?}.", file),
    //     Err(error) => println!("|->  Error creating the file inside the folder bruh {:?}", error),
    // }

    // Error propogation
    fn create_file(file_path: &str) -> Result<File, io::Error> {
        // attempt ot create the file and propogate errors with `?`
        let file = File::create(file_path)?;
        Ok(file)
    }

    match create_file("hello.txt") {
        Ok(_) => println!("|-> The file has been created successfully!."),
        Err(e) => println!("|-> Failed to create the file: {:?}.", e),
    }


}