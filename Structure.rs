use std::io::{self, Write};

struct Student {
    name: String,
    id: i32,
    sgpa: f32,
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn read_int32(prompt: &str) -> i32 {
    loop {
        let input = read_input(prompt);
        match input.parse::<i32>() {
            Ok(i) => return i,
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

fn read_float32(prompt: &str) -> f32 {
    loop {
        let input = read_input(prompt);
        match input.parse::<f32>() {
            Ok(f) => return f,
            Err(_) => println!("Invalid input. Please enter a valid floating-point number."),
        }
    }
}

fn print_student(student: &Student) {
    println!("The name of the student is {} and id is {} and SGPA is {}", student.name, student.id, student.sgpa);
}

fn main() {
    println!("Enter the details of two students:");

    let student_names = vec!["1", "2","3"];
    let mut students = Vec::new();

    for sr in student_names.iter() {
        let name = read_input(&format!("{}. What is your name ? ", sr));
        let id = read_int32(&format!("{}. What is your id ? ", sr));
        let sgpa = read_float32(&format!("{}. What is your sgpa ? ", sr));
        print!("\n");
        students.push(Student { name, id, sgpa });
        
    }

    for student in students.iter() {
        print_student(student);
    }
}






/*



1.The std::io::{self, Write} module is imported for input/output operations.
2.A Student struct is defined with fields name (String), id (i32), and sgpa (f32).
3.The read_input function takes a prompt as input, prints it, reads a line from the standard input, trims any leading/trailing whitespace, and returns the trimmed input as a string.
4.The read_int32 function uses the read_input function to read an integer input from the user. It uses a loop to continuously prompt the user until a valid integer is entered.
5.The read_float32 function is similar to read_int32, but it reads a floating-point number from the user.
6.The print_student function takes a reference to a Student object and prints its details.
7.In the main function, the program prints a message asking the user to enter the details of two students.
8.It initializes a vector student_names with the strings "1", "2", and "3".
9.The program then iterates over the student_names vector, prompts the user for each student's name, id, and SGPA using the read_input, read_int32, and read_float32 functions, and creates a Student object with the provided details. The Student object is then pushed into the students vector.
10.Finally, the program iterates over the students vector and prints each student's details using the print_student function



*/