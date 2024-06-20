fn main() {
    let nutr: i32 = 10;
    println!("The value of num is {}", nutr);

    const NUM: i16 = 15;
    print!("The const value of NUM is {NUM}");
}




/*


1.In the first part of the code, a mutable variable nutr is declared with the type i32 (32-bit signed integer). The value 10 is assigned to nutr, and then it is printed using the println! macro. The {} placeholder is used to insert the value of nutr into the string.
2.In the second part of the code, a constant NUM is declared with the type i16 (16-bit signed integer). The value 15 is assigned to NUM, and then it is printed using the print! macro. The {NUM} placeholder is used to insert the value of NUM into the string.
3.Constants variable are defined in uppercase and they are immutable by default, meaning their values cannot be changed once they are assigned. They are declared using the const keyword, followed by the constant name, a colon, and the type annotation. The value of a constant must be known at compile time, unlike variables, which can be assigned a value at runtime.


*/