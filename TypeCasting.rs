use std::any::type_name;

fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

fn main() {
    
  let x: i32 = 400;
  let y: u32 = x as u32;
  
    println!("{}", x);
    println!("{}", y);
  
    println!("{}", type_of(&x));
    println!("{}", type_of(&y));

}



/*

1.use std::any::type_name; - This line imports the type_name function from the std::any module. This function allows us to get the name of a type at runtime.
2.fn type_of<T>(_: &T) -> &str { type_name::<T>() } - This is a generic function called type_of that takes a reference to any type T and returns a string slice representing the name of the type. It uses the type_name function to get the type name.
3.fn main() {... } - This is the entry point of the program.
4.let x: i32 = 400; - This line declares a variable x of type i32 and assigns the value 400 to it.
5.let y: u32 = x as u32; - This line declares a variable y of type u32 and assigns the value of x (which is of type i32) to y after performing an explicit type cast to u32. This is a safe conversion because the value 400 fits within the range of u32.
6.println!("{}", x); - This line prints the value of x to the console.
7.println!("{}", y); - This line prints the value of y to the console.
8.println!("{}", type_of(&x)); - This line calls the type_of function with a reference to x and prints the returned type name to the console.
9.println!("{}", type_of(&y)); - This line calls the type_of function with a reference to y and prints the returned type name to the console.


*/