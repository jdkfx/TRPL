fn main() {
    let s1 = String::from("welcome!");

    // let (str2, len) = calculate_length(str1);
    let len = calculate_length(&s1);

    println!("{}の長さは、{}です", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize {
    // let length = s.len();
    // (s, length)

    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}