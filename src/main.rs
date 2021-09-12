mod hello_cargo;
fn main() {
    let c = hello_cargo::guess::del();
    println!("Hello, world! {}", c);
}
