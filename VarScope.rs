fn main() {
  let x : i32 = 20;

  {
    let x: i32 = 10;
    println!("The value of scope x is {}",x);
  }
    println!("The value of x is {}", x);
}

/*

1.in scope original value of x is 10 .
2.when we print the value of x again (out of scope) the value is 20.
3.the validity of variable is only in their scope.

*/