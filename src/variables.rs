pub fn run() {
    let name = "Hugo";
    
    let mut age = 37;

    println!("My name is {} and I am {}", name, age);

    age = 38;

    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Hugo", 32);
    println!("{} is {}", my_name, my_age);
}