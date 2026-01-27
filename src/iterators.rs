pub fn iterators() {
    println!("\n||-->    So these are iterators and this is how iterators are used in Rust.");

    let a = [1,33,55];

    let mut iter = a.into_iter();
    println!("||-->    So idk the iterated data, maybe it is: {:?}", iter);

    assert_eq!(Some(1), iter.next());

    // To check iteration
    let mut iter_1 = a.into_iter();
    println!("|-->  So the next iterated value is: {:?}.", iter_1.next());

    assert_eq!(Some(33), iter.next());
    println!("|-->  So the next iterated value is: {:?}.", iter_1.next());
    
    assert_eq!(Some(55), iter.next());

    // And none when its over
    assert_eq!(None, iter.next());

    // More calls may or may not return `None`. Here, they always will.

    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());

    let arr1 = [1,2,3,4]; // One more thing is that arrays are stored in stack.

    let vec1: Vec<i32> = vec![1,2,3]; // And the vectors are stored in heap memory.

    for item in &vec1 {
        // vector ownership is not transferred.
        println!("|-> {}", item);
    }
    println!("\n||-->    {:?}\n", vec1);

    println!("\n||-->    Transfering the ownership\n");

    // vector ownership is being transferred.
    for item in vec1.into_iter() {
        println!("|-> {}", item);
    }

    


}