use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox data");
    }
}

pub fn hello(name: &str) {
    println!("Hello, {name}!");
}

#[cfg(test)]
mod test {
    use super::MyBox;
    use super::hello;

    #[test]
    fn test_pointers() {
        let x = 5;
        let pointer = &x;
        let native_box = Box::new(x);
        let my_box = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *pointer);
        assert_eq!(5, *native_box);
        assert_eq!(5, *my_box);
    }

    #[test]
    fn test_hello() {
        let m1 = "Rust";
        let m2 = MyBox::new(String::from("Rust"));
        let m3 = MyBox::new(String::from("Python"));
        drop(m3);
        hello(&m1);
        hello(&m2);
        // assert_eq!(true, false);
    }
}
