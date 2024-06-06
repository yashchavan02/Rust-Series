fn main() {
  
    println!("Hello \\ world");
  
    println!("Hello \\' world");
  
    println!("Hello \" world");
  
    println!("Hello \n world");
  
    println!("Hello \t world");
  
    println!("Hello \r world");
  
}



/*



1.println!("Hello \\ world") :
This line prints "Hello \ world" because the backslash \ is used as an escape character to represent a literal backslash.
2.println!("Hello \\' world") : 
This line prints "Hello \' world" because the single quote ' is escaped using a backslash \.
3.println!("Hello \" world") : 
This line prints "Hello " world" because the double quote " is escaped using a backslash \.
4.println!("Hello \n world") : 
This line prints "Hello " followed by a newline character and then " world". The \n represents a newline escape sequence.
5.println!("Hello \t world") : 
This line prints "Hello " followed by a tab character and then " world". The \t represents a tab escape sequence.
6.println!("Hello \r world") : 
This line prints " world" because the carriage return \r replaces the beginning of the line.
7.Rust does not support escape sequences like \a, \v, \b, and \f as they are not defined in the Rust language specification.




*/