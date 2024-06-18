fn main() {
    let arr1 = [[1, 2], [3, 4]];
    let arr2 = [[5, 6], [7, 8]];
    let mut add: [[i32; 2]; 2] = [[0; 2]; 2];
    let mut multi: [[i32; 2]; 2] = [[0; 2]; 2];

    println!("{:?}", arr1.len());

    println!("{:?}", arr1);

    println!("{:?}", arr1[0][0]);
    print!("{:?}", arr1[1][1]);

    print!("\n");
    print!("\n");

    for i in 0..2 {
        for j in 0..2 {
            add[i][j] = arr1[i][j] + arr2[i][j];
        }
    }
    for i in 0..2 {
        for j in 0..2 {
            print!("{:?}\t", add[i][j]);
        }
        print!("\n");
    }

    print!("\n");

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                multi[i][j] += arr1[i][k] * arr2[k][j];
            }
        }
    }
    for i in 0..2 {
        for j in 0..2 {
            print!("{:?}\t", multi[i][j]);
        }
        print!("\n");
    }
}



/*



1.The main function is the entry point of the program.
2.Two 2D arrays, arr1 and arr2, are declared and initialized with values [[1, 2], [3, 4]] and [[5, 6], [7, 8]] respectively.
3.Two mutable 2D arrays, add and multi, are declared with the same dimensions as arr1 and arr2 and initialized with zeros.
4.The println! macro is used to print the length of arr1 using the len() method, which returns the number of rows.
5.The println! macro is used to print the entire arr1 using the {:?} format specifier, which displays the array in a readable format.
6.The println! macro is used to print the value at arr1[0][0] and arr1[1][1] using the {:?} format specifier.
7.Nested loops are used to perform addition of arr1 and arr2 and store the result in add.
8.The print! macro is used to print the add array in a tabular format.
9.Nested loops are used to perform multiplication of arr1 and arr2 and store the result in multi.
10.The print! macro is used to print the multi array in a tabular format.




*/