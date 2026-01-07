pub fn referencing() {
    let mut s1 = String::from("Hello barah!");

    let len: usize = calculate_length(&mut s1);

    println!("|| ==>   The length of '{s1}' is {len}.");

}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(", Merry Christmas...");
    s.len()
}