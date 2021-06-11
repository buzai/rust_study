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
    println!("a: {}", a)
}

fn print_arr(a: [i32;2]) {
    println!("{} {}", a[0], a[1]);
}