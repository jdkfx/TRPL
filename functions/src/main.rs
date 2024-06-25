fn main() {
    println!("Hello, world!");

    // 関数の使い方
    another_function(5);
    print_labeled_measurement(5, 'h');

    // 文と式について
    let y = {
        let x = 3;
        x + 1
    };
    println!("yの値は{}", y);

    // 戻り値のある関数
    let five = five();
    println!("fiveの値は{}です", five);

    let six = plus_one(5);
    println!("sixの値は{}です", six);
}

fn another_function(x: i32) {
    println!("xの値は{}です", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("値は{}{}です", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}