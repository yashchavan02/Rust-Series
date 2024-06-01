use std::io;
fn main() {
   let mut inputarray: [i32; 5] = [0; 5];
    println!("Enter the number:");
      for i in  0..5{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read");
        let parsedinput: i32 = match input.trim().parse() {
          Ok(num) => num,
        Err(_) => {
                eprintln!("Invalid input.");
                continue;
            }
        };

      inputarray[i] = parsedinput;
    }

  println!("Output : {:?}", inputarray);
      
}


/*


1.A mutable array inputarray of type i32 with 5 elements is declared and initialized with zeros.
2.The program prompts the user to enter a number using println!("Enter the number:").
3.A for loop is initiated with i ranging from 0 to 4.
4.Inside the loop, a new mutable string input is created.
5.The user input is read into input using io::stdin().read_line(&mut input).expect("Failed to read").
6.The input string is trimmed and parsed into an integer using input.trim().parse().
7.If the parsing is successful, the parsed integer is assigned to parsedinput. If not, an error message is printed using eprintln!("Invalid input."), and the loop continues to the next iteration.
8.The parsed integer is then assigned to the corresponding index of inputarray.
9.After the loop completes, the program prints the final inputarray using println!("Output : {:?}", inputarray).



*/