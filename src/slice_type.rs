pub fn slice_text() {
    let s = String::from("Hello world...");
    let res = find_first_word(&s);


    println!("\n||==>    For string \"{s}\" the Result is {res}");
}

fn find_first_word(input: &String) -> usize {
    let s = input.as_bytes();

    // Print s as bytes - shows the byte values
    println!("Bytes: {:?}", s);
    // Or print each byte individually
    println!("Bytes (individual): {:?}", s.iter().collect::<Vec<_>>());
    // Or print as hex
    println!("Bytes (hex): {:x?}", s);

    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}