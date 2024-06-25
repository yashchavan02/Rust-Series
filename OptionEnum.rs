fn main() {
    let a: MyOption<i32> = MyOption::Some(5); 
    let b: MyOption<&str> = MyOption::Some("Yash Chavan"); 
    let c: MyOption<i32> = MyOption::None;
  
    print!("{:?} {:?} {:?} ", a, b, c);

}
  
#[derive(Debug)]
enum MyOption<T> {
    None,
    Some(T),
}





/*
   
  
1.The MyOption enum has two variants: None and Some(T). The Some variant holds a value of type T, while the None variant represents the absence of a value.
2.In the provided code snippet, the main function demonstrates the usage of the MyOption enum by creating instances of it with different types (i32 and &str). The Some variant is used to wrap the values, while the None variant is used to represent the absence of a value.
3.Finally, the print! macro is used to display the values of the MyOption instances. The {:?} format specifier is used to print the debug representation of the enum variants and their contained values.  
  
  
*/