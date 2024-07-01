pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("ライ麦パン");
    meal.toast = String::from("小麦");
    println!("別のパンを食べます: {}", meal.toast);
    // meal.seasonal_fruit = String::from("ブルーベリー");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("お店の前で行列に並びます");
        }

        pub fn seat_at_table() {
            println!("テーブル席に移動します");
        }
    }

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
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("ピーチ"),
            }
        }
    }
}