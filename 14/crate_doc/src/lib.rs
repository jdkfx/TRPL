//! # Crate doc
//! `crate_doc`は`The Rust Programming Language`のchapter14を元にサンプルとして書いています。

/// 与えられた二つの数値の和を求める。
/// 
/// # Examples
/// ```
/// let result = crate_doc::add(3, 2);
/// assert_eq!(result, 5);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
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
