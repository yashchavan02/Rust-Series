fn main() {
    let x: f64 = 99.02;
    
      assert_eq!(i8::MAX,127);
      assert_eq!(u8::MAX,255);
      assert_eq!(i16::MAX,32767);
     
    
    println!("My CET % tile is {}",x);
    
}






/*
  
1.A variable x of type f64 is declared and assigned the value 99.02.
2.Three assertions are made using the assert_eq! macro. These assertions compare the maximum values of the integer types i8, u8, and i16 with their respective maximum limits:
         1)assert_eq!(i8::MAX, 127); checks if the maximum value of i8 is equal to 127.
         2)assert_eq!(u8::MAX, 255); checks if the maximum value of u8 is equal to 255.
         3)assert_eq!(i16::MAX, 32767); checks if the maximum value of i16 is equal to 32767.
3.After the assertions, a println! macro is used to print a formatted string. The value of x is inserted into the string using the {} placeholder. The output will be: "My CET % tile is 99.02".


*/