fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    println!("arg--------: {}", arg);
    // insp(&arg);
    // insp2(arg.clone()); // u need use clone to copy arg , if not, insp2 will own the arg , so others cant use it, good method is use &, clone is bad
    
    // let ss:&mut String=  &mut arg; or let ss =  &mut arg; 
    // addS(ss);

    // addS(&mut arg);

    // foo(&mut arg);

    // check(arg);

    let a = 1;
    let b = 2;
    let res = sum(&a, &b);
    println!("res: {:?}", res);
}

fn insp(p:&String) {
    println!("{}", p);
}
fn insp2(p:String) {
    println!("{}", p);
}

fn addS(p: &mut String) {
    p.push_str("S");
    println!("{}", p);
}

fn foo(p : &mut String) {
    if !p.ends_with("s") {
        p.push_str("s")
    }
    println!("{}", p);
}

fn check(p : String) {
    if p.starts_with("a") && p.contains("b") {
        println!("true")
    }
}

fn sum(a:&i32, b:&i32) -> i32 {
    a + b
}