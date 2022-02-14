pub fn run() {
    let age: u8 = 32;
    let check_id: bool = true;

    if age >= 21 && check_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave.");
    } else {
        println!("Bartender: I'll need to check your ID.");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false};
    println!("Is of age: {}", is_of_age);
}