pub fn run() {
    let mut hello = String::from("Hello");

    // get length
    let hellostr = "Hello";
    println!("Length: String:{} &str: {}", hello.len(), hellostr.len());
    // mutate String
    // hello.push('a');
    hello.push_str(" aloha");
    // capacity in bytes
    println!("Capacity: {}", hello.capacity());
    // is empty
    println!("Empty: {}", hello.is_empty());
    // Contains
    println!("Contains aloha: {}", hello.contains("aloha"));
    // Replace
    println!("Replace aloha: {}", hello.replace("aloha", " World"));

    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    // string with capacity
    let mut s = String::with_capacity(10);
    s.push_str("Delores");

    // assertions
    assert_ne!(10, s.len(), "Testing that the doesn't have a len of 10");
    assert_eq!(10, s.capacity());

    println!("Papa String: {} &str: {}", hello, hellostr);
}
