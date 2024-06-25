fn main() {
    // 可変な変数の宣言
    let mut x = 5;
    println!("xの値は{}です", x);
    x = 6;
    println!("xの値は{}です", x);

    // 変数の上書き
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("スコープ内でのyの値は{}です", y);
    }
    println!("yの値は{}です", y);
}
