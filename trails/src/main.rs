use trails::Stu;
use trails::Worker;
use trails::Describes;
// use traits::notify;

fn main() {
    // let s = Stu {name:String::from("stu")};
    // println!("{}",s.desc());

    // let w = Worker {name:String::from("worker")};
    // println!("{}",w.desc());

    // s.descDefault();

    // notify(s);
    // notify(w);

    let s = "hello";
    let s0 = "world";
    let s1 = longest(s, s0);
    println!("{}",s);
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else { 
        y
    }
}