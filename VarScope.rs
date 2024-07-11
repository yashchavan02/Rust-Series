fn main() {
  let x : i32 = 20;

  {
    let y : i32 = 10;
    println!("The value of scope x is {}",x);
    println!("The value of scope y is {}",y);
  }
    println!("The value of x is {}", x);
    // println!("The value of y is {}", y);
}


/*

1.in scope original value of x is 10 .
2.when we print the value of y again (In line no. 10) it trow an error -> error[E0425]: cannot find value `y` in this scope.
3.variable y is only valid in block from line no. 4 to 8.
4.the validity of variable is only in their scope.

*/