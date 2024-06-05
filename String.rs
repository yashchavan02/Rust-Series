fn main() {
  

   let my_name_1 : char ='P';
   println!("The character is {}",my_name_1);
     
   let my_name_2 : &str = "Yash";
   println!("My first name is {}",my_name_2);
   
   let mut my_name_3 = String::new();
   my_name_3.push_str("The middle name is Anil");
   println!("{}",my_name_3); 
   
   let my_name_4 = String::from("Chavan");
   println!("My surname is {}",my_name_4);
   
   let namex = "keshav".to_string();
   println!("{}",namex);
   
   let name : &str = "Yash";
   let sliced_name = &name[0..2];
   println!("{}",sliced_name);

   let namey = "keshav".to_string();
   let namez = String::from("Maharaj");
   let concatx = format!("{} {}",namey,namez);
   println!("{}",concatx);

   let namey1 = "keshav".to_string();
   let namez1 = " Maharaj".to_string();
   let concaty = namey1 + &namez1;
   println!("{}",concaty);


}


/*


   
1.Declaring a character variable:
This line declares a variable my_name_1 of type char and assigns the character 'P' to it. It then prints the character using the println! macro.
2.Storing a string literal in a variable:
Here, a string literal "Yash" is stored in a variable my_name_2 of type &str (reference to a string). The println! macro is used to print the string.
3.Creating a mutable string and appending to it:
A mutable string my_name_3 is created using the String::new() function. The push_str method is then used to append a string literal to the mutable string. Finally, the string is printed.
4.Creating a string from a string literal:
A string my_name_4 is created from a string literal "Chavan" using the String::from function. The string is then printed.
5.Converting a string literal to a string:
The to_string method is used to convert a string literal "keshav" to a string namex. The string is then printed.
6.String slicing:
A string name is declared as a reference to a string literal "Yash". String slicing is then performed using the range syntax [0..2] to get the first two characters of the string. The sliced string is printed.
7.String concatenation using the `format!` macro:
Two strings namey and namez are created. The format! macro is used to concatenate the strings with a space in between. The concatenated string is stored in concatx and printed.
8.String concatenation using the `+` operator:
In this case, the + operator is used to concatenate the strings namey1 and namez1. The & operator is used to take a reference to namez1 since the + operator requires a reference to the second operand. The concatenated string is stored in concaty and printed.

   
   
   
*/