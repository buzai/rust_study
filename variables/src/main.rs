fn main() {
    let a = 1;
    let b = 2;
    let c = 3;
    println!("a: {} b: {} c: {} d: {}", a, b, c, a - b);

    let mut e = 0;
    println!("e: {}", e); // if e never use befor assign there will be a warning
    e = 3; // mut is for create a muti varlables
    println!("e: {}", e);
}
