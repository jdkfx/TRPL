extern crate rand;

pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

pub fn add_rand_num(num1: u32, num2: u32) {
    let result1 = num1 + rand::random::<u32>();
    println!("{}にランダムな値を足した結果は{}です", num1, result1);

    let result2 = num2 + rand::random::<u32>();
    println!("{}にランダムな値を足した結果は{}です", num2, result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
