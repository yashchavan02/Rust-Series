fn main() {
  let arr : [i16 ; 9] = [1,2,3,4,5,6,7,8,9];
  for i in arr.iter() {
    print!("{} ",i);
  }
  print!("\n{}\n", arr.len());
  let sliced : &[i16] = &arr[0..=5];
  for i in sliced.iter() {
    print!("{} ",i);
  }
  if sliced.len() != 6 {
     panic!("success!");
  }
  print!("\n");  
}


/* 


1. start code with fn main
2. arr is variable name for array declaration.
3. array is a linear data structure that stores the data in sequencial format.
4. index is start from 0
5. in line no 2 [i16 ; 9] i16 is data type of data present in array and 9 is size.
6. .iter() method for iterate for loop throught array and print the data
7. slicing in array is defined in line no 7 
8. in line no 7 it sclice the data in contiguous manner
9. .len() method used for calculating length of array
10. remember write & befor sclicing ex. &arr[0..=5] and type is &[i16] 


*/
