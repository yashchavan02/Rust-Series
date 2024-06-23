fn main() {
    let a : &str = "Yash Chavan";
    if let "Yash Chavan" = a {
      println!("is present");
    } else {
      println!("is not present");
    }
}





/*
  
  
1.A variable a is declared with a reference to a string literal "Yash Chavan".
2.The if let construct is used to match the value of a against the string literal "Yash Chavan".
3.If the match is successful, the program prints "is present".
4.If the match fails, the program prints "is not present".


*/