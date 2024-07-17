fn main(){

    let name1 : String = String::from("I am a Student");
    let mut name2 : String = String::new();

    name2.push_str(" I am a Student ");
    
    println!("{}",name1.replace("Student","Programmer"));
    
    for n in name1.lines() {
        println!("{}",n);
    }
    
    for n in name1.split(" ") {
        println!("{}",n);
    }
    
    println!("{}",name2.trim());
    
    for n in name1.chars() {
        println!("{}",n);
    }

    if let Some(n) = name1.chars().nth(3) {
        println!("{}", n);
    }

    if let None = name1.chars().nth(3) {
        println!("No char is their.");
    } else {
        println!("Char is their.");
    }
    
}




/*



1.name1.replace("Student","Programmer"): This replaces all occurrences of "Student" with "Programmer" in the name1 string.
2.for n in name1.lines(): This iterates over each line in name1 and prints it.
3.for n in name1.split(" "): This splits name1 into words based on spaces and prints each word.
4.name2.trim(): This removes leading and trailing whitespace from name2 and prints the trimmed string.
5.for n in name1.chars(): This iterates over each character in name1 and prints it.
6.if let Some(n) = name1.chars().nth(3): This checks if the fourth character exists in name1 and prints it if it does.
7.if let None = name1.chars().nth(3): This checks if the fourth character does not exist in name1 and prints a message accordingly.



*/