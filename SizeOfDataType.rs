fn main() {

    let a = 10_i32; 
    println!("{} bytes are used for {} (i32)", std::mem::size_of_val(&a), a);
    
    let b = 20.5_f64; 
    println!("{} bytes are used for {} (f64)", std::mem::size_of_val(&b), b);
      
    let c = a as isize;
    println!("{} bytes are used for {} (isize)", std::mem::size_of_val(&c), c);
    
    let d = b as i8;
    println!("{} bytes are used for {} (i8)", std::mem::size_of_val(&d), d);
      
    let e = b as u16;
    println!("{} bytes are used for {} (u16)", std::mem::size_of_val(&e), e);

    let f = a as u32;
    println!("{} bytes are used for {} (u32)", std::mem::size_of_val(&f), f);

    let g = a as f32;
    println!("{} bytes are used for {} (f32)", std::mem::size_of_val(&g), g);
      
    let h = a as f64;
    println!("{} bytes are used for {} (f64)", std::mem::size_of_val(&h), h);
      
    let i : char = 'Y';
    println!("{} bytes are used for {} (char)", std::mem::size_of_val(&i), i);
      
    let j = a > 0;
    println!("{} byte is used for {} (bool)", std::mem::size_of_val(&j), j);

    let k : String = String::from("Yash Chavan");
    println!("{} bytes are used for {} (String)",std::mem::size_of_val(&k),k);

}



/*


1.let a = 10_i32; --> Declares an integer variable a with a value of 10 and explicitly specifies its type as i32.
2.println!("{} bytes are used for {} (i32)", std::mem::size_of_val(&a), a); --> Prints the memory size of a and its value.
3.let b = 20.5_f64; --> Declares a floating-point variable b with a value of 20.5 and explicitly specifies its type as f64.
4.println!("{} bytes are used for {} (f64)", std::mem::size_of_val(&b), b); --> Prints the memory size of b and its value.
5.let c = a as isize; --> Performs an integer to integer conversion from a to isize.
6.println!("{} bytes are used for {} (isize)", std::mem::size_of_val(&c), c); --> Prints the memory size of c and its value.
7.let d = b as i8; --> Performs a floating-point to integer conversion from b to i8.
8.println!("{} bytes are used for {} (i8)", std::mem::size_of_val(&d), d); --> Prints the memory size of d and its value.
9.let e = b as u16; --> Performs a floating-point to integer conversion from b to u16.
10.println!("{} bytes are used for {} (u16)", std::mem::size_of_val(&e), e); --> Prints the memory size of e and its value.
11.let f = a as u32; --> Performs an integer to integer conversion from a to u32.
12.println!("{} bytes are used for {} (u32)", std::mem::size_of_val(&f), f); --> Prints the memory size of f and its value.
13.let g = a as f32; --> Performs an integer to floating-point conversion from a to f32.
14.println!("{} bytes are used for {} (f32)", std::mem::size_of_val(&g), g); --> Prints the memory size of g and its value.
15.let h = a as f64; --> Performs an integer to floating-point conversion from a to f64.
16.println!("{} bytes are used for {} (f64)", std::mem::size_of_val(&h), h); --> Prints the memory size of h and its value.
17.let i : char = 'Y'; --> Declares a character variable i with a value of 'Y'.
18.println!("{} bytes are used for {} (char)", std::mem::size_of_val(&i), i); --> Prints the memory size of i and its value.
19.let j = a > 0; --> Declares a boolean variable j by comparing a with 0.
20.println!("{} byte is used for {} (bool)", std::mem::size_of_val(&j), j); --> Prints the memory size of j and its value.
21.let k : String = String::from("Yash Chavan"); --> Declares a string variable k with a value of "Yash Chavan".
22.println!("{} bytes are used for {} (String)",std::mem::size_of_val(&k),k); --> Prints the memory size of k and its value.



*/
