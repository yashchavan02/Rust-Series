use std::io;
fn main() {
    println!("Enter what you want:");
    let mut input =String::new();
        io::stdin()
        .read_line(& mut input)
        .expect("Fail");

//     let x: &str =input.trim();
//     println!("{}",x);

    println!("You enter : {}", input.trim());
}


/*

1.The use std::io; statement imports the io module from the standard library, which provides input/output functionality.
2.The fn main() {... } block defines the entry point of the program.
3.Inside the main function, the println! macro is used to display a message to the user: "Enter what you want:".
4.The let mut input = String::new(); statement declares a mutable variable input of type String and initializes it with an empty string.
5.The io::stdin().read_line(&mut input).expect("Fail"); line reads a line of input from the standard input (keyboard) and stores it in the input variable. The expect("Fail"); part is an error handling mechanism that will print "Fail" and terminate the program if an error occurs during input reading.
6.The let x: &str = input.trim(); line creates a reference to a string slice (&str) from the input string, trimming any leading or trailing whitespace.
7.The println!("{}", x); line prints the trimmed string slice x.
8.The println!("You enter : {}", input.trim()); line prints the trimmed input string, along with the message "You enter : ".


*/