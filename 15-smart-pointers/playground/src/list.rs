use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[cfg(test)]
mod test {
    use super::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_list_creation() {
        let _ = Cons(
            Rc::new(RefCell::new(1)),
            Rc::new(Cons(
                Rc::new(RefCell::new(2)),
                Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::new(Nil))),
            )),
        );
    }

    #[test]
    fn test_list_with_multiple_owners() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(
            Rc::new(RefCell::new(5)),
            Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))),
        ));

        println!("count after creating a = {}", Rc::strong_count(&a));

        let _b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));

        *value.borrow_mut() += 1000;

        println!("a after = {a:?}");
        println!("b after = {_b:?}");

        {
            let _c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
            println!("c after = {_c:?}");
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));

        // panic!();
    }
}
