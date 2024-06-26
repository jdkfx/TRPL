fn main() {
    let mut s = String::from("You will find it.");

    let (first_index, first_word) = first(&s);

    let sub_str = &s[first_index+1..];

    let binding = sub_str.to_string();
    let (_second_index, second_word) = first(&binding);

    println!("最初の単語は{}で、次の単語は{}です", first_word, second_word);
    s.clear();
}

fn first(s: &String) -> (usize, &str) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (i, &s[0..i]);
        }
    }

    (s.len(), &s[..])
}
