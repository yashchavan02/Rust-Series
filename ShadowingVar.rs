fn main() {
  let x: i32 = 5;
  {
    let x: i32 = 10;
   println!("the value of VarScope is {}", x);
  }
  println!("the value of x original {}",x);
  let x: i32 = 45;
  print!("the value of x after shadowing {}",x);
}


/*

1.use variable again with same name and different value is known as shadowing.
2.in line no. 4 redeclaring the x variable then redeclaring in line no. 8 but the value of x [2nd line] is 5 , value of x [4th line] is 10 and  value of x [8th line] is 45.
3.owership is not tranfer in shadowing.
4.code line 3 to 6 is 'VarScope' concept.

*/