#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("a生成後のカウント = {}", Rc::strong_count(&a));

    // let b = Cons(3, Rc::clone(&a));
    // println!("b生成後のカウント = {}", Rc::strong_count(&a));

    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("c生成後のカウント = {}", Rc::strong_count(&a));
    // }
    // println!("cがスコープを抜けた後のカウント = {}", Rc::strong_count(&a));
    
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
