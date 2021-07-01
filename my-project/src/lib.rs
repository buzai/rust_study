// mod front_of_house {

//     pub struct User {
//         name : String,
//     }
//     pub struct BreakFast {
//         pub toast: String,
//         fruit: String,
//     }

//     impl BreakFast {
//         pub fn summer(toast: &str) -> BreakFast {
//             BreakFast { 
//                 toast: String::from(toast),
//                 fruit: String::from("peaches"),
//             }
//         }
//     }

//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {
//             super::serving::take_order()
//         }
//     }
//     pub mod serving {
//         pub fn take_order() {

//         }
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }


// use crate::front_of_house::hosting;
// pub use crate::front_of_house::serving;


// pub fn eat_house() {
//     crate::front_of_house::hosting::add_to_waitlist();
//     front_of_house::hosting::add_to_waitlist();
//     hosting::add_to_waitlist();

// }   

// pub fn eat() {
//     let mut meal = front_of_house::BreakFast::summer("ry");
//     meal.toast = String::from("wheat");
//     println!("{}", meal.toast);

//     // meal.fruit = String::from("fur");
// }

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn test() {
    front_of_house::hosting::host();
    hosting::host();
}