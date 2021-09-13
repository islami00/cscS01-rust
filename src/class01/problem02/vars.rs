pub fn run(){
    let name = "Delores";
    let mut age =  32;
    println!("My name is {0} and I am {1}",name,age);
    age = 33;
    println!("My name is {0} and I am {1}",name,age);

    // define constant
    const ID: i32 = 13;
    println!("ID : {}",ID);
    
    // destructuring assignment
    let (my_name,my_age,delores) = ("Islam",23,"Random");
    println!("My name is {0} and I am {1}, and you? {2}",my_name,my_age,delores);


}