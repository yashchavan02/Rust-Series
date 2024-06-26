fn main() {
    let college1: Colleges = Colleges::Coep;
    let college2: Colleges = Colleges::Pict;
    let college3: Colleges = Colleges::Vit;
    let college4: Colleges = Colleges::Pccoe;

    let color1: Color = Color::Black(String::from("Fav"));
    let color2: Color = Color::White(100);
    let color3: Color = Color::Skyblue(50.50);

    match color1 {
        Color::Black(ref s) => print!("{:?} ", s),
        _ => (),
    }
    match color2 {
        Color::White(ref i) => print!("{:?} ", i),
        _ => (),
    }
    match color3 {
        Color::Skyblue(ref f) => print!("{:?} ", f),
        _ => (),
    }
    
    println!("");
    
    let colleges = [college1, college2, college3, college4];

    for college in colleges.iter() {
        print!("{:?} ", college);
    }
}

#[derive(Debug)]
enum Colleges {
    Coep,
    Pict,
    Vit,
    Pccoe,
}

#[derive(Debug)]
enum Color {
    Black(String),
    White(usize),
    Skyblue(f64),
}





/*



1.The Colleges enum is defined with four variants: Coep, Pict, Vit, and Pccoe.
2.The Color enum is defined with three variants: Black, White, and Skyblue. Each variant has a different associated type: String for Black, usize for White, and f64 for Skyblue.
3.Four instances of the Colleges enum are created: college1, college2, college3, and college4.
4.Three instances of the Color enum are created: color1, color2, and color3.
5.Pattern matching is used to extract and print the values associated with the Color enum instances.
6.An array of Colleges instances is created: colleges.
7.The for loop iterates over the colleges array and prints each value using the {:?} format specifier.
8.Enum data type allocate common memory for all instances.
 



*/