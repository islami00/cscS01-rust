pub fn run() {
    // positional args
    println!(
        "Hello {0} your {1} has {2} {0}!",
        "delores", "peacock", "died"
    );
    // named args
    println!(
        "Hello {del} your {pet} has {state} wih name: {pet}!",
        del = "delores",
        pet = "peacock",
        state = "died"
    );
    // placeholder traits
    println!("binary: {0:b} hex : {0:x} octal: {0:o}", 10);
    // palceholder for debug traits
    println!(
        "Delores's friends: {:?}",
        ("Abbie the dresser", "who has", 5, "cats",)
    );
    // math
    println!("Two cats and dogs rain: {}", 5 + 5);
}
