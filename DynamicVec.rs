fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(0);

    v.remove(0);

    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(50);

    v.extend([60, 70, 80, 90]);

    println!("Size of vec is {}", v.len());

    println!("{:?}", v);

    println!("The value at index 1 is {}", v[1]);

    for i in v {
        print!("{i} \t");
    }
}



/*



1.A mutable vector v of type i32 is declared and initialized using Vec::new().
2.The push method is used to add elements to the vector. In this case, the value 0 is pushed to the vector.
3.The remove method is used to remove an element from the vector at the specified index. In this case, the element at index 0 (which is 0) is removed.
4.More elements are added to the vector using the push method. The values 10, 20, 30, 40, and 50 are pushed to the vector.
5.The extend method is used to add multiple elements to the vector at once. An array [60, 70, 80, 90] is extended to the vector.
6.The len method is used to get the size of the vector and it is printed using println!.
7.The entire vector is printed using println! with {:?} to display its contents.
8.The value at index 1 of the vector is accessed using indexing (v[1]) and it is printed using println!



*/