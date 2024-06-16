fn main() {
  
    let mut v: Vec<i32> = vec![1, 2, 3, 4,5];
    println!("{:?}", v);
  
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
  
    v.extend([10,11,12,13,14,15]);
  
    for _ in 1..8{
      v.remove(v.len()-1);
    }
  
    println!("{:?}", v);

}



/*
 

1.A mutable vector v of type i32 is declared and initialized with the values [1, 2, 3, 4, 5]. The vec! macro is used to create the vector.
2.The println! macro is used to print the initial state of the vector v.
3.Four elements (6, 7, 8, 9) are added to the vector using the push method.
4.Six additional elements (10, 11, 12, 13, 14, 15) are added to the vector using the extend method. The extend method takes an iterable (in this case, an array) and appends its elements to the vector.
5.A loop is used to remove the last seven elements from the vector. The remove method is used to remove an element at a specific index. In this case, the loop iterates from 1 to 8 (exclusive), and the index passed to remove is v.len() - 1, which removes the last element.
6.Finally, the println! macro is used to print the final state of the vector v.

 
*/