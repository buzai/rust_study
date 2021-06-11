fn main() {
    foo();

    let a = 1;
    let b = 2;
    let res =sum(a, b);
    println!("res: {}", res);
}

fn foo() {
    println!("Hello, foo func!");
}

fn sum(a: u32, b: u32) -> u32 {
    println!("a+b: {}", a+b);
    a+b
}