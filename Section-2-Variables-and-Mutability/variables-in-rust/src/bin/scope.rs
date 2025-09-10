fn main() {
    let num = 21;

    {
        let new_num = 12;
        println!("My luck number is {new_num}");
        // println!("My luck number is {num}"); -> Accesible
    }

    println!("My luck number is {num}");
    // println!("My luck number is {new_num}"); -> Not Accesible
}
