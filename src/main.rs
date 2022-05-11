use stack::{ simple::SimpleStack};

mod stack;

fn main() {
    let mut e = SimpleStack::new();
    e.push(22).unwrap();
    e.push(123).ok();
    e.push(23).ok();
    e.push(1111).ok(); // Should err for next two
    match e.push(12345) {
        Ok(_) => {

            let _ = e.pop();
            println!("Yo {:?}", e.peak());
        }
        eth => {
            
            println!("Yo {:?}, peak: {}",eth, e.peak().unwrap() );
        },
    };
}
