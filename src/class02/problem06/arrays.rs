use std::mem;
pub fn run() {
    let mut arr: [i32; 5] = [1, 3, 4, 5, 6];
    //  printing arrs
    println!("This is the whole thing {:?}", arr);
    // get single values
    println!(
        "This is the first {},2nd {},3rd {},4th {},5th {}",
        arr[0], arr[1], arr[2], arr[3], arr[4]
    );

    // reassign
    arr[1] = 23;
    // len
    println!("len: {}", arr.len());

    // Arrays are stack allocated
    let size = mem::size_of_val(&arr);
    println!("This array occupies {} bytes in memory", size);

    // Get slice
    let slice: &[i32] = &arr[0..2];
    println!("Slice: {:?}", slice);
}
