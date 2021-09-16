pub fn run() {
    let name: &str = "Negritto";
    let courses_gpa: Vec<[&str; 2]> = vec![["C++", "b"], ["C++", "b"], ["C++", "b"], ["C++", "b"]];
    let gpa_no: [(&str, i8); 6] = [("a", 5), ("b", 4), ("c", 3), ("d", 2), ("e", 1), ("f", 0)];
    println!("{:?}", gpa_no);
    let gp_store: Vec<i8> = vec![];
    for course in courses_gpa.iter() {
        for gp in gpa_no.iter() {}
    }
}
