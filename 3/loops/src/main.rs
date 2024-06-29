fn main() {
    // 無限ループ
    // loop {
    //     println!("Hi!");
    // }

    use_loop_label();
    use_while();
    use_for();
}

fn use_loop_label() {
    // ループラベル
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn use_while() {
    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn use_for() {
    let a = [10, 20, 30, 40, 50];

    // let mut index = 0;
    // while index < 5 {
    //     println!("配列の値は{}です", a[index]);
    //     index += 1;
    // }

    for element in a {
        println!("配列の値は{}です", element);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("LIFTOF!!!");
}