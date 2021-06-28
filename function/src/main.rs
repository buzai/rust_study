fn main() {
    foo();

    let a = 1;
    let b = 2;
    let res =sum(a, b);
    println!("res: {}", res);

    // fpp();
    // fppp();
    // foor();
    // fos();
    let mut s = String::from("some string");
    // mfn(s.clone());
    // println!("{}",s);

    // yinf(&s);

    // yinf2(&mut s);
    // println!("{}",s);
    
    // let res = firststr(&s);

    let res = firststr2(&s[..]);
    println!("{}",s);
    // s.clear();
    println!("{}",res);

}

fn foo() {
    println!("Hello, foo func!");
}

fn sum(a: u32, b: u32) -> u32 {
    println!("a+b: {}", a+b);
    a+b
}

fn fpp() {
    let y = {
        let x= 1;
        x + 1
    };
    println!("{}", y);
}

fn fppp() -> u32 {
    3
}

fn ifs(a : u32) {
    if a > 0 {
        println!(">0");
    } else {
        println!("<0");
    }

    let x = true;
    let b = if x  { 1 } else { 0 };
    println!("{}",b);


}

fn looo() {
    let mut index = 1;
    let res = loop {
        index = index + 1;
        if index == 10 {
            break index * 2
        }
    };
    println!("{}",res);
}

fn foor () {
    let a = [1,2,3,4,5];
    for item in a.iter() {
        println!("{}", item);
    }

    // rang 
    for item in (1 .. 4) {
        println!("{}", item);
    }

    for item in (1 .. 4).rev() {
        println!("{}", item);
    }
}

fn fos() {
    let mut s = String::from("hello");
    s.push_str(" world");
    // let s1 = s;
    let s1 = s.clone();

    println!("{}", s1);
}

fn mfn(mut s: String) {
    s.push_str("fos");
    println!("{}", s);
}

fn yinf(s : &String) {
    // s.push_str("fos");
    
    println!("{}", s);
}

fn yinf2(s : &mut String) {
    s.push_str("fos");
    
    println!("{}", s);
}

fn first(s : &String) ->usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn firststr(s : &String) ->&str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[ .. i];
        }
    }
    return &s[..];
}

fn firststr2(s : &str) ->&str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[ .. i];
        }
    }
    return &s[..];
}

fn s() {
    let arr = [1,2,3,4,5,6];
    let a = &arr[1 .. 2]; // a type is &[i32]
}