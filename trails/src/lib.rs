use std::fmt::Display;

pub trait Describes {
    fn desc(&self) -> String;
    fn descDefault(&self) {
        println!("default");
    }
}



pub struct Stu {
    pub name: String,
}

impl Describes for Stu {
    fn desc(&self) -> String {
        format!("stu {}", self.name)
    }
}

pub struct Worker {
    pub name: String,
}

impl Describes for Worker {
    fn desc(&self) -> String {
        format!("worker {}", self.name)
    }
}

// pub fn notify(item : impl Describes) {
//     println!("notify: {}", item.desc());
// }

// pub fn notify(item : impl Describes + Display) {
//     println!("notify: {}", item.desc());
// }

pub fn notify<T: Describes>(item: T) {
    println!("notify: {}", item.desc());
}

// pub fn notify<T: Describes + Display>(item: T) {
//     println!("notify: {}", item.desc());
// }

// pub fn notify2<T, U>(item: T)
//      where T: Describes + Display, U: Describes
//  {
//     println!("notify: {}", item.desc());
// }

// pub fn notify2<T: Describes>(item: T) -> impl Describes
//  {
//     println!("notify: {}", item.desc());
//     item
// }

