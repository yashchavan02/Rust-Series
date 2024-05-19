fn main() {

    let a: i32 = 5;

    assert_eq!(a, 5);
    assert_ne!(a, 0);

    assert!(a==5);
    assert!(a!=0);
    assert!(a < 7);
    assert!(a <= 8);
    assert!(a > 3);
    assert!(a >= 5);

    print!("Hello Coders");

}

/*

1.assert_eq! --> if both values are equal then return true otherwise return false.
2.assert_ne! --> if both values are not equal then return true otherwise return false.
3.assert! --> (true) then return true otherwise return false.
4.let keyword use for initialise the variable. 
  Example : in above code {a :i32 = 5} --> "a" is variable name , i32 is variable type [integer] and "5" is variable value.

*/