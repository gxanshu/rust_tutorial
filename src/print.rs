pub fn run() {
    println!("Hello from print.rs file");

    println!("{}", 1);

    println!("age is: {} and name is {}", 19, "Anshu");

    // positiional indexing
    println!("{name} is a good person", name = "anshu");

    // binary indexing
    println!("{:b}, {:x}, {:o}", 10, 10, 10);

    // debug traits
    println!("{:?}", (12, true, "anshu"));
}
