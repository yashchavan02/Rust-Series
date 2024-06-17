fn main() {
    let mut t:(i32,f32,bool,char) = (12,23.5,true,'f');
    let u = (132,"Yash Chavan",false,'g');
   
       println!("{:?}",t);

       t.0 = 25;
       println!("{:?}",t.0);
       println!("{:?}",t.1);
       println!("{:?}",t.2);
       println!("{:?}",t.3);
       
       println!("{:?}",u);

    let (a , _b , _c , _d) = u;

       println!("{:?}",a);  

    let p = u;

       println!("{:?}",p);

}
 



/*


1.Declaration and initialization of a mutable tuple t with four elements: an integer (i32), a floating-point number (f32), a boolean (bool), and a character (char). The tuple is initialized with values (12, 23.5, true, 'f').
2.Declaration and initialization of a tuple u with four elements: an integer, a string, a boolean, and a character. The tuple is initialized with values (132, "Yash Chavan", false, 'g').
3.Printing the tuple t using the {:?} format specifier, which prints the debug representation of the tuple.
4.Modifying the first element of the tuple t by assigning a new value of 25.
5.Printing the individual elements of tuple t using the {:?} format specifier.
6.Printing the tuple u using the {:?} format specifier.
7.Deconstructing the tuple u into individual variables using pattern matching. In this case, the first element is assigned to the variable a, and the other elements are ignored using the underscore (_) pattern.
8.Printing the value of a.
9.Assigning the tuple u to a new variable p.
10.Printing the value of p.



*/ 