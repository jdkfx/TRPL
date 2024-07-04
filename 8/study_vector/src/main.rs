fn main() {
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    let first: &i32 = &v[0];
    let _third: &i32 = &v[2];
    println!("一つ目の要素は{}です", first);

    match v.get(2) {
        Some(third) => 
            println!("三つ目の要素は{}です", third),
        None => println!("三つ目の要素はありません"),
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }
}
