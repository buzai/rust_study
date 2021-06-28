fn main() {
    let tups: (i32, i32) = (1, 2);
    println!("{}", tups.0);
    println!("{}", tups.1);

    let arr: [i32;2] = [1;2];
    println!("{}", arr.len());
    println!("{}", arr.last().unwrap());
    println!("{}", arr[0]);
    print_arr(arr);


    let com: (i32, u32) = (1, 2);
    println!("{}", com.0);
    println!("{}", com.1);

    let series = [1, 1, 2, 3, 5, 8, 13];

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");

    let flag:bool = true;
    if flag {
        println!("is true");
    }

    let a = {
        1 + 1
    };
    println!("a: {}", a);

    let a1: u32 = "42".parse().expect("not number");
    println!("a1: {}", a1);

    // 2^n -1 ~ 2^n-1 -1 / 0 ~ 2^n
    let f1 = 0.1; // f1 is f64 
    let f2 : f32 = 0.1; // f2 f32

    let b1 = true;
    let mut b2: bool;
    b2 = false;
    b2 = true;
    println!("b2 {}", b2);


    let t = (1, 2);
    let (t1, t2)=t;
    // t.0 t.1

    let arrr = [1,24];

    let arrrrr : [u32; 5];
    let arrrrrr = [3; 50];

    println!("{}", arrrrrr[30]);
    println!("{}", arrrrrr[49]);


}

fn print_arr(a: [i32;2]) {
    println!("{} {}", a[0], a[1]);
}