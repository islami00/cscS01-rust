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
    // testing hypothesis for Vec<u8> string type
    let na: Vec<u8> = vec![23u8, 42u8, 4u8];
    let ya: Vec<u8> = vec![23u8, 42u8, 4u8];
    // testing for the enum
    #[derive(Debug, PartialEq, Clone)]
    pub enum Social {
        Twitter(Vec<u8>),
        Facebook(Vec<u8>),
        Instagram(Vec<u8>),
        Riot(Vec<u8>),
        Email(Vec<u8>),
        None,
    }
    // testing if the absunique works
    use std::mem::{discriminant, Discriminant};
    type ProjectSocials = Vec<Social>;
    pub trait AbsUnique {
        /// Check if a vector contains duplicate instances of an enum variant, regardless of data stored
        fn abstr_dup(&self) -> bool;
    }
    impl AbsUnique for ProjectSocials {
        fn abstr_dup(&self) -> bool {
            // memo for the discriminants
            let mut x: Vec<Discriminant<Social>> = Vec::new();
            // copy of self for iter
            let y = (&self).to_vec();
            let mut dupl = false;

            // loop
            for n in y.iter() {
                // Functions take type arguments as ::<>
                let desc = discriminant::<Social>(n);
                if x.contains(&desc) {
                    dupl = true;
                    break;
                };
                x.push(desc);
            }
            dupl
        }
    }
    let rnd: ProjectSocials = vec![Social::Email(vec![1]), Social::Email(vec![2])];
    let isrnd = rnd.abstr_dup();
    // data for enum eq test
    let cads = Social::Twitter(vec![2, 3]);
    let cads2 = Social::Twitter(vec![1, 2, 3]);
    // testing destructuring
    if let Social::Twitter(i) = &cads {
        println!("Is this it? {:?}", i)
    }
    // testing enum eq
    let x2 = cads.eq(&cads2);
    // testing Vec<u8> eq
    let x = na.eq(&ya);
    let z = this_vector;
    println!("Copy? {:?}", z);
    // items with copy trait  can be copied by simply reffing them elsewhere, won't go out of scope.
    println!("Were they equal? {:?}", x);
    println!("Is this a string? {:?}", cads);
    println!("Were the two abstracted equal? {:?}", x2);
    println!("Does it have duplicates? {:?}", isrnd)
}
