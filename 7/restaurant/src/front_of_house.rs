// mod front_of_house {
pub mod hosting;

pub mod serving {
    pub fn take_order() {
        println!("注文を伺います");
    }

    pub fn serve_order() {
        println!("料理を出します");
    }

    // mod back_of_house {
    //     pub struct Breakfast {
    //         pub toast: String,
    //         seasonal_fruit: String,
    //     }

    //     impl Breakfast {
    //         pub fn summer(toast: &str) -> Breakfast {
    //             Breakfast {
    //                 toast: String::from(toast),
    //                 seasonal_fruit: String::from("ピーチ"),
    //             }
    //         }
    //     }

    //     fn fix_incorrect_order() {
    //         cook_order();
    //         super::serve_order();
    //     }
    
    //     fn cook_order() {
    //         println!("料理を作ります");
    //     }
    // }

    pub fn take_payment() {
        println!("料金を支払います");
    }
}
// }
