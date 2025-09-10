fn main() {
    let apples = 21;
    let oranges = 12;
    let _fruit = apples + oranges;
    let is_admin = true;
    let user = "Harsh";

    println!("I have {} apples", apples);
    println!("I have {oranges} oranges");
    println!("I have {apples} apples and {oranges} oranges");
    println!("I have {} apples and {} oranges", apples, oranges);
    println!("I have {0} apples and {1} oranges", apples, oranges);
    println!("Is admin {is_admin}");
    println!("{user} is User");
}
