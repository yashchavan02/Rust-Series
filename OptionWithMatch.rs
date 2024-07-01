fn main() {
    let a: Option<i32> = None;
    let a1: Option<i32> = square(a);
    println!("The square of 0 is {:?}", a1);

    for number in 1..10 {
        let a: Option<i32> = Some(number);
        let a1: Option<i32> = square(a);
        println!("The square of {number} is {:?} ", a1);
    }
}

fn square(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * i),
    }
}




/*


1.In this code snippet, the main function initializes an Option<i32> variable a with None. It then calls the square function with a as an argument and assigns the result to a1. The println! macro is used to print the result, which in this case is "The square of 0 is None".
2.Next, a loop is used to iterate over the numbers from 1 to 9. Inside the loop, a new Option<i32> variable a is initialized with Some(number). The square function is called again with a as an argument, and the result is assigned to a1. The println! macro is used to print the result, which in this case is "The square of {number} is Some(number_squared)".
3.The square function is defined separately and takes an Option<i32> as an argument. It uses a match expression to handle the two possible cases of the Option type: None and Some(i). If the input is None, the function returns None. If the input is Some(i), the function calculates the square of i and returns Some(i * i).


*/
