fn main() {
    let mut numx = 1;

    for num in 1..10 {
        if num == 3 {
            continue;
          }
        if num == 5 {
            break;
          }
        println!("{}", num);
    }

    print!("\n");

    while numx != 10 {
        if numx == 3 {
            numx += 1;
            continue;
           }
        if numx == 5 {
            break;
           }
        println!("{}", numx);
        numx += 1;
    }
}






/*



1.The main function is defined, which is the entry point of the program.
2.Inside the main function, a mutable variable numx is declared and initialized with the value 1.
3.A for loop is used to iterate over the range 1..10. For each iteration, the current value of num is checked:
   1) If num is equal to 3, the continue statement is executed, which skips the rest of the current iteration and moves on to the next iteration.
   2) If num is equal to 5, the break statement is executed, which exits the loop immediately.
   3) If neither of the above conditions is met, the value of num is printed using println!.
4.After the for loop, a newline character is printed using print!("\n") to separate the output of the two loops.
5.A while loop is used to iterate while numx is not equal to 10. For each iteration, the current value of numx is checked:
   1) If numx is equal to 3, numx is incremented by 1 using numx += 1, and the continue statement is executed, which skips the rest of the current iteration and moves on to the next iteration.
   2) If numx is equal to 5, the break statement is executed, which exits the loop immediately.
   3) If neither of the above conditions is met, the value of numx is printed using println!, and numx is incremented by 1.



*/
