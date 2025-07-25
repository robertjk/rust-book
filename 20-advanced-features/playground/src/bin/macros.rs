use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[macro_export]
macro_rules! vec_custom {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn declarative_macro() {
    let vec_custom = vec_custom![1, 2, 3];
    println!("vec custom: {vec_custom:?}");
}

fn procedural_macro() {
    #[derive(HelloMacro)]
    struct Pancakes;

    Pancakes::hello_macro();
}

fn main() {
    declarative_macro();
    procedural_macro();
}
