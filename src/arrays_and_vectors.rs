pub fn arrays() {
    // SO BASICALLY THE ARRYAS ARE THE COLLECTION OF NUMBERS OR DATA TYPES, WHICH ARE ARRANGED IN STRAIGHT ORDER.
    println!("|-->    So these are arrays in Rust language.");

    let a1: [i64; 5];
    // SO now we will initialize the array a1 by adding the values to it.
    a1 = [10,10,10,10,19];

    println!("|-->    So the array stored in a1 is: {:?}", a1);

    // Declaring mutable arrays
    let mut a2 = [32; 5];
    a2[2] = 99;
    a2[4] = 67;
    println!("|-->    So the third value of the a2 array is: {}. and the first value is {}. Hence the last value which we changed is now: {}.", a2[2], a2[0], a2[4]);
}

pub fn vectors() {

    // SO BASICALLY VECTORS ARE ARRAYS BUT THESE ARE DYNAMIC ARRAYS WHERE WE CAN CHANGE IT, THAT CAN GROW AND SHRINK IN SIZE AT RUNTIME!!!.
    println!("\n\n|-->    So these are vector in the Rust language");

    let mut v: Vec<i32> = Vec::new(); // First way of creating an empty vector
    let mut v1 = Vec::<i32>::new(); // Second way of creatinf an empty vector

    // Creating vector with initial values
    let value_vec = vec![1,2,3,4,5]; // Creates vector with initial values.

    // create vector with 5 elements initialized of 0 only and nothing else.
    let auto_vec: Vec<i32> = vec![0;5];
    println!("So vector initialized is: {:?}.", auto_vec);

    let mut _v1: Vec<i32> = Vec::new();
    let mut _v2= Vec::<i32>::new();

    let mut _v3 = vec![100,101,111];

    let opt = _v3.get(10);

    match opt {
        Some(value) => println!("Value is : {}", value),
        None => println!("No value"),
    }

    _v3.push(23);
    _v3.push(99);

    println!("||-->    So the pushed elements in the array _v3 is: {:?}", _v3);
    println!("||-->    So now we will pop the last element from the array _v3.");

    _v3.pop();
    println!("||-->    So now the last element of the array _v3 which is \"99\" will be poped: {:?}.", _v3);

    // ALSO THERE IS A METHOD WHERE YOU CAN INSERT A PERTICULAR ITEM AT PARTICULAR PLACE OR INDEX IN THE ARRAY, we use ".insert(<index to be placed at>,<the data type you want to insert>)". which is very helpful for complex operations.
    // .insert()
    _v3.insert(0, 69);
    println!("||-->    This is the array now after inserting the item in the first place in _v3: {:?}.", _v3);

    // to print individual elements present in the list we use for loop to iterate through it.
    let mut count = 1;
    for item in &_v3 {
        println!("{}. {}.", count, item); // So this is how you iterate through the list using for loop and print each element present in the list/array.
        count += 1;
    }
    

}