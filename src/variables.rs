pub fn run() {
    let name = "anshu";
    let mut age = 19;
    println!("{}", age);
    age = 20;
    println!("{}", age);

    // const
    /**
     * the difference between const and nomutable variables you have to spacify const with there proper type
     */

    const ID: i32 = 001;
    println!("{}", ID);

    // you can asign multiples variables at the same time;

    let (age, my_work) = (19, "JS developer");

    println!("my age{} ane my work {}", age, my_work);

    println!("{} is good", name);
}
