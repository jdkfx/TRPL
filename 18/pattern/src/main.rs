struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum At {
    Hello { id: i32 },
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("あなたのお気に入りの色、{}を背景色に使用します", color);
    } else if is_tuesday {
        println!("火曜日は緑の日");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("紫を背景色に使用します");
        } else {
            println!("オレンジを背景色に使用します");
        }
    } else {
        println!("青を背景色に使用します");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("index {}: value {}", index, value);
    }

    let point = (3, 7);
    show_point(&point);

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("any"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Someは50でした"),
        Some(y) => println!("y = {:?}でマッチしました", y),
        _ => println!("デフォルトのケース i = {:?}", x),
    }
    println!("最後の値 x = {:?}, y = {:?}", x, y);

    let x = 1;
    match x {
        1 | 2 => println!("1 or 2"),
        3 => println!("3"),
        _ => println!("any"),
    }

    let x = 4;
    match x {
        1..=5 => println!("1 ~ 5"),
        _ => println!("something"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("ASCII文字前半"),
        'k'..='z' => println!("ASCII文字後半"),
        _ => println!("それ以外"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    match p {
        Point { x, y: 0 } => println!("x軸上の{}", x),
        Point { x: 0, y } => println!("y軸上の{}", y),
        Point { x, y } => println!("どちらの軸上でもない: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("Quit列挙子には分配すべきデータがない")
        },
        Message::Move { x, y } => {
            println!("x方向に{}, y方向に{}だけ動く", x, y)
        },
        Message::Write(text) => println!("テキストメッセージ: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("色を赤{}, 緑{}, 青{}に変更", r, g, b)
        }
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();
    println!("sum: {}", sum_of_squares);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("設定を上書きできません");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("設定は{:?}です", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("なんらかの値: {} {} {}", first, third, fifth)
        },
    }

    let s = Some(String::from("hello"));
    if let Some(_) = s {
        println!("文字列が見つかりました");
    }
    println!("{:?}", s);

    let origin = Point { x: 32, y: 54 };
    match origin {
        Point { x, .. } => println!("x: {}", x),
    }

    match numbers {
        (first, .., last) => {
            println!("{}から始まり, {}で終わる", first, last);
        },
    }

    let mut robot_name = Some(String::from("Rob"));
    match robot_name {
        Some(ref mut name) => *name = String::from("another name"),
        None => (),
    }
    println!("robot_name: {:?}", robot_name);

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("5未満です: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("50です"),
        Some(n) if n == y => println!("n = {:?}でマッチしました", n),
        _ => println!("デフォルトのケース, x = {:?}", x),
    }
    println!("最終的な値: x = {:?}, y = {:?}", x, y);

    let x = 5;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let msg = At::Hello { id: 5 };
    match msg {
        At::Hello { id: id_variable @ 3..=7 } => {
            println!("範囲内のidが見つかりました: {}", id_variable)
        },
        At::Hello { id:  10..=12 } => {
            println!("別の範囲内のidが見つかりました")
        },
        At::Hello { id } => {
            println!("それ以外のidが見つかりました")
        },
    }
}

fn show_point(&(x, y): &(i32, i32)) {
    println!("現在地: ({}, {})", x, y);
}
