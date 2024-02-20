// Copyright (c) Cookie Yang. All right reserved.
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    next: Option<Rc<Node<T>>>,
    prev: RefCell<Option<Weak<Node<T>>>>,
    data: T,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, data: T) -> LinkedList<T> {
        let new_node = Rc::new(Node {
            data,
            next: self.head.clone(),
            prev: RefCell::new(None),
        });
        match self.head.clone() {
            Some(node) => {
                let mut prev = node.prev.borrow_mut();
                *prev = Some(Rc::downgrade(&new_node));
            }
            Node => {}
        }

        LinkedList {
            head: Some(new_node),
        }
    }
}

fn main() {
    ///原始指针
    let a = &56;
    let a_raw_ptr = a as *const i32;
    let mut b = &mut 555;
    let mut b_mut_ptr = b as *mut i32;
    b = &mut 33;

    ///Box<T>
    let box_one = Box::new(Foo);
    let un_box_one = *box_one;
    box_ref(un_box_one);

    ///引用计数
    let list_of_number = LinkedList::new().append(1).append(2);
    println!("{:?}", list_of_number);

    println!("end");
}

fn box_ref<T>(b: T) -> Box<T> {
    let a = b;
    Box::new(a)
}

struct Foo;
