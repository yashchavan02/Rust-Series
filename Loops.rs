fn main() {
    let mut i: i32 = 1;
    'iterate : loop{
      println!("{}",i);
        i=i+1;
      if i==10{
        break 'iterate;
      }
    }
  }



/*

1.The fn main() function is the entry point of the program.
2.Inside the main function, a mutable integer variable i is declared and initialized with the value 1.
3.A labeled loop is defined with the label 'iterate.
4.Inside the loop, the value of i is printed using println!("{}", i).
5.The value of i is incremented by 1 using i = i + 1.
6.An if statement checks if i is equal to 10.
7.If i is equal to 10, the break keyword is used with the lable 'iterate to exit the loop.

*/