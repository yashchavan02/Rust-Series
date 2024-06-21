fn main() {
    let condition : bool = false;
    let number = if condition {23} else {32};
    println!("The value of number is {}",number);

    let condition : bool = true;
    let number = if condition {45} else {67};
    println!("The value of number is {}",number);
}



/*


1.Two variables condition and number are declared. The condition variable is initially set to false. The number variable is then assigned a value using an if-else statement. If the condition is true, the value 23 is assigned to number; otherwise, the value 32 is assigned. The program then prints the value of number.
2.After that, the condition variable is set to true, and the if-else statement is executed again. This time, if the condition is true, the value 45 is assigned to number; otherwise, the value 67 is assigned. The program then prints the value of number.


*/