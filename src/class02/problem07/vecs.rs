use std::mem;
pub fn run() {
    let mut this_vector: Vec<i32> = vec![1, 3, 4, 5, 6];
    //  printing vectors
    println!("This is the whole thing {:?}", this_vector);
    // get single values
    println!(
        "This is the first {},2nd {},3rd {},4th {},5th {}",
        this_vector[0], this_vector[1], this_vector[2], this_vector[3], this_vector[4]
    );

    // reassign
    this_vector[1] = 23;

    // add onto it.
    this_vector.push(23);
    this_vector.push(23);

    // pop off
    this_vector.pop();
    // len
    println!("len: {}", this_vector.len());

    // vectors are stack allocated
    let size = mem::size_of_val(&this_vector);
    println!("This vector occupies {} bytes in memory", size);

    // Get slice
    let slice: &[i32] = &this_vector[0..2];
    println!("Slice: {:?}", slice);

    println!("This is the whole thing {:?}", this_vector);

    // looping through
    for x in this_vector.iter() {
        println!("What {}", x)
    }
    // loop and mutate
    for x in this_vector.iter_mut() {
        *x *= 2
    }
    println!("Done eh? {:?}", this_vector)
}
