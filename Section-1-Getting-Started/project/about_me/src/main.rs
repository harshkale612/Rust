/*
Create a new `about_me` project with the `cargo new` command.

Using the `println!` macro, output 3 sentences about yourself.
Feel free to invoke the macro multiple times.

From the Terminal, compile the `main.rs` file inside the `src`
folder with the Rust compiler, then manually run the executable.

From the Terminal, compile the project with the Cargo tool, then
manually run the executable.

From the Terminal, compile and run the project with a single
Cargo command.

Check your program for errors with `cargo check`.

Add a comment at the top of your source code explaining how to
compile the program for new Rust developers.

Add some spaces and line breaks to the code so that it is formatted
in an ugly manner. From the Terminal, style the code with the
`cargo fmt` command.

Replace the `println!` macro with `print!`. What happens?
*/

// For compiling and executing the code use "cargo run" command in terminal

fn main() {
    print!("I am Harshavardhan");
    print!("Rustacean");
    print!("I love writting code in rust")
}

/*
user@linux:~/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me/src$ rustc main.rs
user@linux:~/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me/src$ ./main
I am Harshavardhan
Rustacean
I love writting code in rust
user@linux:~/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me/src$ cd ..
user@linux:~/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me$ cargo build
   Compiling about_me v0.1.0 (/home/user/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
user@linux:~/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me$ cd target/debug/
user@linux:~/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me/target/debug$ ls
about_me  about_me.d  build  deps  examples  incremental
user@linux:~/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me/target/debug$ ./about_me
I am Harshavardhan
Rustacean
I love writting code in rust
user@linux:~/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me/target/debug$ cd ..
user@linux:~/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me/target$ cd ..
user@linux:~/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/about_me`
I am Harshavardhan
Rustacean
I love writting code in rust
user@linux:~/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me$ cargo check
    Checking about_me v0.1.0 (/home/user/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
user@linux:~/Desktop/Harsh-Code/Programming-In-Rust/Rust/Section-1-Getting-Started/project/about_me$ cargo fmt

*/
