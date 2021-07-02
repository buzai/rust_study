#[derive(Debug)]
enum CellItem {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let vc : Vec<i32> = Vec::new(); // new create a empty vector
    // vc.push(1);
    // println!("{:?}",vc);

    // let vc = vec![1,2,3,4];
    // println!("{:?}",vc);
 
    // let mut vc: Vec<i32> = Vec::new();
    // // let mut vc= Vec::new();
    // vc.push(1);
    
    // println!("{:?}",vc);

    // let item : &i32 = &vc[0];
    // println!("{:?}",item);

    // match vc.get(0) {
    //     Some(v) => println!("get value {}",v),
    //     None => println!("none")
    // }

    // for i in &mut vc {
    //     println!("{:?}",i);
    //     *i+= 10;
    // }
    // for i in &mut vc {
    //     println!("{:?}",i);
    // }

    let row = vec![
        CellItem::Int(1),
        CellItem::Float(1.0),
        CellItem::Text(String::from("::1")),
    ];

    for item in row {
        println!("{:?}", item);
    }
}
