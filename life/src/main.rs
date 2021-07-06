struct Point <T>{
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x0(&self) -> &T {
        &self.x
    }
}

impl Point<i32> {
    fn x1(&self) -> &i32 {
        &self.x
    }
}

#[derive(Debug)]
struct Point2 <T, U>{
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn m<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 { x: self.x, y: other.y }
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }



fn main() {

    // let v = vec![1, 2, 3];
    // let mut largest = v[0];
    // for item in v {
    //     if item > largest {
    //         largest = item;
    //     }
    // }
    // println!("{}", largest);

    // let v = vec![1, 2, 3];
    // let l = get_largest(&v);
    // println!("{}", l);

    // let i = Point { x:1, y:2 };
    // let i2 = Point { x:1.1, y:2.1 };

    // let i3 = Point2 { x:1, y:1.0 };

    let p1 = Point2 { x:1, y:2.0 };
    let p2 = Point2 { x: "hello", y: "world"};
    let p3 = p1.m(p2);
    println!("{:?}", p3);
}

fn get_largest(v: &[i32]) -> i32 {
    let mut largest = v[0];
    for &item in v {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest<T: std::cmp::PartialOrd>(v: &[T])->T{
//     let mut largest = v[0];
//     for &item in v {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }