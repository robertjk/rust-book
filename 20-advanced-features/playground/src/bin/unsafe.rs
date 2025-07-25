use std::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    raw_pointers_valid();
    // raw_pointers_invalid();

    unsafe_function_or_method_1();
    unsafe_function_or_method_2();

    external_code();

    static_variables_unmutable();
    static_variables_mutable();

    unsafe_trait();
}

pub fn raw_pointers_valid() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

#[allow(dead_code)]
fn raw_pointers_invalid() {
    // Accessing the following pointer will throw an error
    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r is: {}", *r);
    }
}

pub fn unsafe_function_or_method_1() {
    unsafe fn dangerous() {
        println!("Dangerous function");
    }

    unsafe {
        dangerous();
    }
}

pub fn unsafe_function_or_method_2() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn external_code() {
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
        safe fn floor(n: f64) -> f64;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    println!("Floor of 5.67 according to C: {}", floor(5.67));
}

fn static_variables_unmutable() {
    println!("name is: {HELLO_WORLD}");
}

fn static_variables_mutable() {
    /// SAFETY: Calling this from more than a single thread at a time is undefined
    /// behavior, so you *must* guarantee you only call it from a single thread at
    /// a time.
    unsafe fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

fn unsafe_trait() {
    #[allow(dead_code)]
    unsafe trait Foo {
        // Methods go here
    }

    unsafe impl Foo for i32 {
        // Method implementions go here
    }
}
