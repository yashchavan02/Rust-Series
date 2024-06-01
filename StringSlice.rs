fn main() {
    let x: String = String::from("hereissometextformatwordsrandomtext");
      
    let y=&x[..];
    println!("{}",y);
      
    let y=&x[..5];
    println!("{}",y);
      
    let y=&x[7..];
    println!("{}",y);
      
    let y=&x[2..9];
    println!("{}",y);
    
}


/*


1.A string x is declared and initialized with the value "hereissometextformatwordsrandomtext".
2.A reference y is created to the entire string x using the slice syntax &x[..]. This means y points to the entire string.
3.The println! macro is used to print the value of y. As a result, the entire string "hereissometextformatwordsrandomtext" is printed.
4.Another reference y is created to the first 5 characters of x using the slice syntax &x[..5].
5.The println! macro is used to print the value of y. As a result, the first 5 characters "herei" are printed.
6.A third reference y is created to the substring starting from the 7th character of x using the slice syntax &x[7..].
7.The println! macro is used to print the value of y. As a result, the substring starting from the 7th character "sometextformatwordsrandomtext" is printed.
8.A fourth reference y is created to the substring from the 2nd character to the 9th character of x using the slice syntax &x[2..9].
9.The println! macro is used to print the value of y. As a result, the substring "hereis" is printed.



*/