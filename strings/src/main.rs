fn main() {
    // string is bytes
    // string
    // let mut s = String::new();
    // println!("{}", s);

    // let str = "hello";
    // println!("{}", str);
    // let s = str.to_string();
    // println!("{}", s);
    // let s1 = "hello".to_string();

    // let s = String::from("hello");


    // let mut s  = String::new();
    // s.push_str("hello");
    // s.push('l');
    // println!("{}", s);

    // let s2 = String::from("world");

    // let s3 = s + &s2;
    // println!("{}", s3);

    // let s1 = String::from("a");
    // let s2 = String::from("b");
    // let s = String::from("s");
    // let s3 = s + "-" + &s1 + &s2;
    // println!("{}", s3);

    // let s1 = String::from("a");
    // let s2 = String::from("b");
    // let s = String::from("s");
    // let s3 = format!("{}={}={}", s1, s2, s);
    // println!("{}", s3);

    let s1 = String::from("heool");
    let len = s1.len();
    println!("{}", len);

    // for b in s1.bytes() {
    //     println!("{}", b);
    //     println!("--------------------------------")
    // }

    // for c in s1.chars() {
    //     println!("{}", c);
    // }

    let s2 = &s1[0..2];
    println!("{}", s2);

}
