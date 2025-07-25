fn main() {
    function_pointers();
    function_or_closure();
    returning_closures();
}

fn function_pointers() {
    assert_eq!(do_twice(add_one, 5), 12);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn function_or_closure() {
    let list_of_numbers = vec![1, 2, 3];

    let list_of_string1: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_string2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    assert_eq!(list_of_string1, vec!["1", "2", "3"]);
    assert_eq!(list_of_string2, vec!["1", "2", "3"]);

    #[derive(PartialEq, Debug)]
    #[allow(dead_code)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses1: Vec<Status> = (0u32..5).map(Status::Value).collect();
    let list_of_statuses2: Vec<Status> = (0u32..5).map(|val| Status::Value(val)).collect();

    assert_eq!(
        list_of_statuses1,
        vec![
            Status::Value(0),
            Status::Value(1),
            Status::Value(2),
            Status::Value(3),
            Status::Value(4)
        ]
    );
    assert_eq!(
        list_of_statuses2,
        vec![
            Status::Value(0),
            Status::Value(1),
            Status::Value(2),
            Status::Value(3),
            Status::Value(4)
        ]
    );
}

fn returning_closures() {
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |x| x + init)
    }

    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("handler output: {output}");
    }
}
