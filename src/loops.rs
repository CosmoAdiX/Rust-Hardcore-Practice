use colored::*;

pub fn loops() {

    let mut n1: u8 = 1;

    println!("hello from main fn");// this is a single being printed

    // and here is method to print infinitely
    let result = loop {
        println!("Bruh am being printed infinitely bruh!...      {n1}x");

        if n1 == 10 {
            break 69;
        }
        n1 = n1 + 1;
    };
    println!("\n {} {result}","======================== This is the end. ========================".green());
}

pub fn while_loops() {
    let mut num: i8 = 0;

    while num != 10 {
        println!("{num}!");

        num += 1;
    }
    println!("\n {}","The loop have stopped...".red());
}

pub fn while_using_arrays() {
    let array1: [i32; 6] = [1,2,3,4,5,6];
    let mut index = 0;

    while index < 6 {
        println!("{}{}{}{}","index: ".red(), index.to_string().green(), ", and value: ".red(), array1[index].to_string().green());
        index += 1;
    }
}

pub fn for_loop() {
    let array2: [i64; 18] = [1,2,3,4,5,6,3,4,3,43,43,4,99,34,4,34,4,4343];


    for x in array2 {
        println!("x: {x}");
    }
    println!("{}","======================== For Loop Ends Here ========================".bright_yellow());
}

pub fn ranges_in_for_loop() {
    // to use ranges in for loop:
    for x in 1..=10 {
        println!("x: {x}");
    }

    println!("\n {}","======================== Reversed For Loop ========================".bright_green());

    for x in (1..=10).rev() {
        println!("x reversed: {x}");
    }
}