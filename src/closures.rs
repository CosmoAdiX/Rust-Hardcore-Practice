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
    let y = consume_and_return(); // the value of the str_1 gets transfer orm closure to the variable y.
    println!("|->  So the string is, \"{}\".", y);

    let z = y;
    println!("|->  So now the ownership of the string str_1 gets transferred to the z from y, \"{}\".", z);

    println!("\n|->  Lets now implement it wiht vectors using closures in process");

    let vector_1 = vec![22,24,33,34,45,46];

    let even_num: Vec<i32> = vector_1.into_iter().filter(|x| x % 2 == 0).collect();

    println!("|->  So the new array is, {:?}", even_num);
    
    let vec2 = vec![1,2,3,4];

    let double_vec: Vec<i32> = vec2.iter().map(|x| x * 2).collect();
    println!("|->  {:?}", double_vec);

    let even_vec: Vec<&i32> = vec2.iter().filter(|x| *x % 2 == 0).collect();
    println!("|->  {:?}", even_vec);
}