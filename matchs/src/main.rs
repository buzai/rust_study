fn main() {
    // println!("Hello, world! {}", match_foo(Step::second));
    // println!("Hello, world! {}", match_foo(Step::three(ST::F)));
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // match some if let
    let x = Some(9);
    if let Some(9) = x {
        println!("match")
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn foo(s: Step) {
    match s {
        Step::first => println!("first"),
        _ => println!("not match")
    }
}

#[derive(Debug)]
enum ST {
    F,
    S,
}

enum Step {
    first,
    second,
    three(ST),
}

fn match_foo(s: Step) -> u32 {
    match s {
        Step::first => 1,
        Step::second => {
            println!("2");
            2
        },
        Step::three(st) => {
            println!("{:?}", st);
            println!("3");
            3
        },
    } 
}