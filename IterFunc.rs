fn main() {
    let number = [105;12];
  
      for n in number.iter() {
         print!("{}\t",n);
      }
}



/*


1.An array named number is declared with 12 elements, all initialized to the value 105. The syntax [105; 12] is used to create an array with a specific size and initialize all elements with the same value.
2.The iter() method is called on the number array to create an iterator. This iterator allows us to iterate over the elements of the array.
3.A for loop is used to iterate over the elements of the number array. The n variable is used to hold the current element during each iteration.
4.Inside the loop, the print! macro is used to print the value of the current element (n) followed by a tab character (\t). The {} placeholder is used to insert the value of n into the string.
5.The loop continues until all elements of the number array have been printed.



*/