fn main() {

      println!("{}",1+2);
      println!("{}",2-2);
      println!("{}",1*2);
      println!("{}",12/2);
      println!("{}",13%3);
      

      println!("{}",1<2);
      println!("{}",2!=2);
      println!("{}",1==2);
      println!("{}",2>=2);
    

      println!("{}",1==1 && 2==2);
      println!("{}",2>=2 || 1!=1);
      println!("{}",!(1==2));


      println!("{}", 0b1010 & 0b1100); 
      println!("{}", 0b1010 | 0b1100);     
      println!("{}", 0b1010 ^ 0b1100);
      println!("{}", !0b1010);
      println!("{}", 0b1010 << 2);
      println!("{}", 0b1010 >> 2); 
    
      
      let mut number :i32 = 20;
      number+=5;
      println!("{}",number);
      number-=5;
      println!("{}",number);
      number*=5;
      println!("{}",number);
      number%=5;
      println!("{}",number);
    
      
}




/*




1.Arithmetic Operators:
- println!("{}",1+2); --> Prints the result of the addition operation (1 + 2 = 3).
- println!("{}",2-2); --> Prints the result of the subtraction operation (2 - 2 = 0).
- println!("{}",1*2); --> Prints the result of the multiplication operation (1 * 2 = 2).
- println!("{}",12/2); --> Prints the result of the division operation (12 / 2 = 6).
- println!("{}",13%3); --> Prints the remainder of the division operation (13 % 3 = 1).
2.Comparison Operators:
- println!("{}",1<2); --> Prints true as 1 is less than 2.
- println!("{}",2!=2); --> Prints false as 2 is not equal to 2.
- println!("{}",1==2); --> Prints false as 1 is not equal to 2.
- println!("{}",2>=2); --> Prints true as 2 is greater than or equal to 2.
3.Logical Operators:
- println!("{}",1==1 && 2==2); --> Prints true as both conditions are true.
- println!("{}",2>=2 || 1!=1); --> Prints true as at least one condition is true.
- println!("{}",!(1==2)); --> Prints true as the negation of 1==2 is false.
4.Bitwise Operators:
- println!("{}", 0b1010 & 0b1100); --> Prints the result of the bitwise AND operation (1010 AND 1100 = 1000).
- println!("{}", 0b1010 | 0b1100); --> Prints the result of the bitwise OR operation (1010 OR 1100 = 1110).
- println!("{}", 0b1010 ^ 0b1100); --> Prints the result of the bitwise XOR operation (1010 XOR 1100 = 0110).
- println!("{}",!0b1010); --> Prints the result of the bitwise NOT operation (NOT 1010 = 0101).
- println!("{}", 0b1010 << 2); --> Prints the result of the left shift operation (1010 shifted left by 2 bits = 101000).
- println!("{}", 0b1010 >> 2); --> Prints the result of the right shift operation (1010 shifted right by 2 bits = 101).
5.Compound Operators:
- let mut number :i32 = 20; --> Declares a mutable integer variable number and assigns it the value 20.
- number+=5; --> Adds 5 to the value of number (20 + 5 = 25).
- println!("{}",number); --> Prints the updated value of number (25).
- number-=5; --> Subtracts 5 from the value of number (25 - 5 = 20).
- println!("{}",number); --> Prints the updated value of number (20).
- number*=5; --> Multiplies the value of number by 5 (20 * 5 = 100).
- println!("{}",number); --> Prints the updated value of number (100).
- number%=5; --> Calculates the remainder of the division of number by 5 (100 % 5 = 0).
- println!("{}",number); --> Prints the updated value of number (0).





*/    