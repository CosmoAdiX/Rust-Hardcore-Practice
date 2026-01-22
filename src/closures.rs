pub fn closures() {
    println!("\n||-->    These are closures in the rust language\n|");

    let add_one = |x: i32| x+1;
    println!("||-->    So the value is: {}.", add_one(69));
    println!("||-->    So the value is: {}.", add_one(11));
    println!("||-->    So the value is: {}.", add_one(12));
    println!("||-->    So the value is: {}.", add_one(223));

    let mut counter = 0;

    let mut increase_counter = || {
        counter = counter + 1;
        println!("|->  {}", counter)
    };

    increase_counter();
    increase_counter();
    increase_counter();

    let str_1 = String::from("wht the phuck amigo?");
    let consume_and_return = || str_1;
    // println!("|->  {}", str_1);
    let y = consume_and_return();
    println!("|->  So the string is, \"{}\".", y);
    
}