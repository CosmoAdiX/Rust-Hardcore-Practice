pub fn take_ownership() {
    let x: i32 = 11;
    let result = add(x);

    let name = String::from("Aditya Kumar");
    let s = gives_ownership();
    let s: String = takes_and_gives_back_ownership(s);

    takes_ownership(name);

    println!("The added num is {x} and the result is==> {result}");
    println!("s ==> {s}");
}

pub fn calulate_len(s: String) -> (String, usize) {
    let result = s.len();
    (s, result)
}

fn gives_ownership() -> String {
    let s: String = String::from("This is a string from gives_ownership.");
    s
}

fn takes_ownership(s: String) {
    println!("Inside Ownership ==> {s}");
}

fn takes_and_gives_back_ownership(s: String) -> String {
    println!("S in takes_and_gives_back_ownership: {s}");
    s
}

fn add(x: i32) -> i32 {
    x + 10
}
