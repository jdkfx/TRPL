extern crate another_one;

fn main() {
    let num1 = 5;
    let num2 = 12;
    println!("{} + {} = {}", num1, num2, another_one::add(num1, num2));
    another_one::add_rand_num(num1, num2);
}
