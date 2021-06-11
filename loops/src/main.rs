fn main() {
    
    let args : Vec<String> = std::env::args().skip(1).collect();
    println!("len : {}", args.len());
    for arg in args {
        println!("arg: {}", arg);
        if arg == "sum" {
            sum();
        }
        if arg == "loop" {
            lop();
        }
    }
    
}

fn sum() {
    println!("sum ~~~~~~~~");
}

fn lop() {
    let mut a = 1;
    loop {
        a += 1;
        if a == 10 {
            break;
        }
    }
    println!("{}", a);
}