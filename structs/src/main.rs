fn main() {
    let dog = Dog {
        name: String::from("nik"),
        age: 12
    };
    println!("{}", dog.age);
    println!("{}", dog.name);

    let age = 1;
    let dog2 = Dog {
        name: String::from("nik2"),
        age // dont need key "age"
    };

    let dog3 = Dog {
        name: String::from("nik3"),
        ..dog // others value from dog
    };
    println!("{:?}", dog3);
    println!("{:#?}", dog3);

    let black = color(1,2,3);
    println!("{}", black.1);

    let r = React{
        width: 10,
        height: 20
    };
    println!("{:?}", r);
    println!("{}", r.area());

    let mut r2 = React{
        width:1,
        height:2
    };
    println!("{}", r2.area());
    r2.addWidth(12); // like instance method
    println!("{}", r2.area());

    React::structfn(); // like oop class method
}
#[derive(Debug)]
struct Dog {
    name : String,
    age: u32
}

struct color(u8, u8, u8); // tup struct 

#[derive(Debug)]
struct React {
    width: u32,
    height: u32
}

impl React {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn addWidth(&mut self, width: u32) -> u32 {
        self.width += width;
        return self.width;
    }
}

impl React {
    fn structfn() {
        println!("struct fn")
    }
}

struct unitStruct;