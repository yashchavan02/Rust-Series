fn main(){

    let s1 : String = String::from("Hello Rust Coders");
    let s2 : String = s1;
    println!("{}",s2);

    let s1 : String = String::from("Hello Rust Coders");
    let s2 : String = s1.clone();
    println!("{}",s1);
    println!("{}",s2);

    let mut s1 = String::new();
    s1.push_str("Hello Rust Coders");
    let s2 : &str = &s1;
    println!("{}",s1);
    println!("{}",s2);

}



/*


1.Lines 03-05 : A String is created and assigned to s1. When s1 is assigned to s2 without using the clone method, ownership of s1 is transferred to s2. This means that s1 is no longer valid and cannot be used after this assignment. When s2 is printed, it successfully displays the string "Hello Rust Coders".
2.Lines 07-10 : A new String is created and assigned to s1. The clone method is used to create a copy of s1 and assign it to s2. Both s1 and s2 now have their own copies of the string. When s1 and s2 are printed, they both display the string "Hello Rust Coders".
3.Lines 12-16 : A new empty String is created and assigned to s1 using String::new(). The push_str method is used to append a string to s1. A reference to s1 is then assigned to s2 using &s1. Since s1 is mutable, it can still be used after the reference is created. When s1 and s2 are printed, they both display the string "Hello Rust Coders".


*/