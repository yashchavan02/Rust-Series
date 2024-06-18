fn main() {
    let mut numx = 1;

    for num in 1..10 {
        if num == 3 {
            continue;
          }
        if num == 5 {
            break;
          }
        println!("{}", num);
    }

    print!("\n");

    while numx != 10 {
        if numx == 3 {
            numx += 1;
            continue;
           }
        if numx == 5 {
            break;
           }
        println!("{}", numx);
        numx += 1;
    }
}
