pub fn run() {
    //  println!("Delores")

    // default is i32

    let i = 32;
    // default f64
    let y = 2.5;

    // explicit typinh
    let d: i64 = 444444444444444444;

    // max values
    println!("i32 primitive max is: {}", std::i32::MAX);
    println!("i64 primitive max is: {}", std::i64::MAX);

    // booleans
    let is_active = true;

    // get from expr
    let is_greater = 10 > 5;
    let ca = '\u{1F600}';
    println!("x, y,z isactive isgreater char{:?}", (i, y, d, is_active, is_greater, ca));

    // char
}
