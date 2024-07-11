fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let number_list = vec![34, 54, 29, 100, 64];
    let result = largest(&number_list);
    println!("最大値は{}です", result);

    let char_list = vec!['d', 't', 'n', 'q'];
    let result = largest(&char_list);
    println!("最大値は{}です", result);

    let str1 = String::from("dafksj");
    let str2 = "xfg";
    let result = longest(str1.as_str(), str2);
    println!("長い文章は{}です", result);
}
