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