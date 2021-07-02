use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

fn main() {
    // let file = File::open("hello.txt");
    // let f = match file {
    //     Ok(file) => {
    //         println!("get file");
    //         file
    //     },
    //     Err(err) => {
    //         println!("some error {:?}", err);
    //         match err.kind() {
    //             ErrorKind::NotFound => match File::create("hello.txt") {
    //                 Ok(fc) => fc,
    //                 Err(e) => panic!("error creat {:?}", e),
    //             },
    //             other_err => panic!("error open {:?}", other_err),
    //         }
    //     }
    // };

    // let f = File::open("hello.txt").unwrap();

    // let f = File::open("hello.txt").expect("file error");

    let res = read_files();
    println!("{:?}", res);

    let res = read2();
    println!("{:?}", res);

    let res = read3();
    println!("{:?}", res);
}

fn read_files() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string( &mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
