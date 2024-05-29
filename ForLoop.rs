fn main() {
    let mut sum:i32 =0;
    
      for i in 0..=5{
      sum=sum+i;
         }
    
   assert_ne!(sum,10);
    
    for j in 'a'..='z'{
      println!("{}",j);
         }
    
}




/*



1.Loops:
    1)The first loop is a for loop that iterates over a range from 0 to 5 (inclusive) using the ..= range syntax.
    2)Inside the loop, the variable sum is incremented by the current value of i.
    3)After the loop finishes, an assertion is made using assert_ne! to check if the final value of sum is not equal to 10. If the assertion fails, the program will panic.
2.Loops:
    1)The second loop is another for loop that iterates over a range of characters from 'a' to 'z' (inclusive) using the ..= range syntax.
    2)Inside the loop, each character j is printed to the console using println!.
3.For loop syntax -->
           [for] <variable> in <range>{
                 <code block>
            }



*/