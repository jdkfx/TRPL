fn main() {
    // if
    let num1 = 7;
    if num1 < 5 {
        println!("条件は真でした");
    } else {
        println!("条件は偽でした");
    }

    // else if
    let num2 = 6;
    if num2 % 4 == 0 {
        println!("4で割り切れます");
    } else if num2 % 3 == 0 {
        println!("3で割り切れます");
    } else if num2 % 2 == 0 {
        println!("2で割り切れます");
    } else {
        println!("4,3,2で割り切れません");
    }
}
