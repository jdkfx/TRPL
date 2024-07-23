use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// use List::{Cons, Nil};

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match *self {
//             Cons(_, ref item) => Some(item),
//             Nil => None,
//         }
//     }
// }

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // println!("leafの親 = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leafのstrong_count = {}, weak_count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branchのstrong_count = {}, weak_count = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        
        println!(
            "leafのstrong_count = {}, weak_count = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leafの親 = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leafのstrong_count = {}, weak_count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    // println!("aの最初の参照カウント = {}", Rc::strong_count(&a));
    // println!("aの次の要素 = {:?}", a.tail());

    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    // println!("b作成後のaの参照カウント = {}", Rc::strong_count(&a));
    // println!("bの最初の参照カウント = {}", Rc::strong_count(&b));
    // println!("bの次の要素 = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // println!("aを変更後のbの参照カウント = {}", Rc::strong_count(&b));
    // println!("aを変更後のaの参照カウント = {}", Rc::strong_count(&a));

    // println!("aの次の要素 = {:?}", a.tail());
}
