fn main() {
    let data = "新しい文字列データ";
    let mut s = data.to_string();

    // let s = "新しい文字列データ".to_string();
    // let s = String::from("新しい文字列データ");

    s.push_str(", 追加の文字列データ");
    println!("{}", s);

    // push は一文字を引数にとるため、文字を '' で囲む
    s.push('!');
    println!("{}", s);

    let right = String::from("Hello, ");
    let left = String::from("World!");
    let result = right + &left;
    println!("{}", result);

    let red = String::from("red");
    let yellow = String::from("yellow");
    let green = String::from("green");
    // let traffic_light = red + "-" + &yellow + "-" + &green;
    let traffic_light = format!("{}-{}-{}", red, yellow, green);
    println!("{}", traffic_light);

    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }

    for c in "Здравствуйте".bytes() {
        println!("{}", c);
    }
}
