fn main() {

    let mut a : Vec<char> = vec!['z','a','b','c','d','e','f','g','h'];

    a.push('i');
    a.remove(0);

       for c in &a {
         print!("{:?} \t",c)
       }

    print!("\n");

    a.pop();

  let s: String = a.into_iter().collect();
  println!("Char Vec to String : {}", s);

  println!("String length is {}",s.len());
}


/*



1.A mutable vector a of characters is declared and initialized with a list of characters.
2.The push method is used to add the character 'i' to the end of the vector.
3.The remove method is used to remove the first element ('z') from the vector.
4.A for loop is used to iterate over a reference to the vector a and print each character.
5.After the loop, the pop method is used to remove the last element ('i') from the vector.
6.The into_iter method is used to create an iterator over the vector a.
7.The collect method is used to convert the iterator into a String.
8.The resulting string is printed, showing the conversion of the character vector to a string.
9.Finally, the length of the string is printed using the len() method.




*/