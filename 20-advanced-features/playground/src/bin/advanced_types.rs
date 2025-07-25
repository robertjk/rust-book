fn main() {
    dynamically_sized_types();
}

#[allow(dead_code)]
fn never_type_1() -> ! {
    panic!();
}

#[allow(dead_code)]
fn never_type_2() -> ! {
    loop {
        print!("and ever ");
    }
}

fn dynamically_sized_types() {
    // let s: str = "Hello there";
    let s: &str = "Hello there";

    println!("s: {s}");
}
