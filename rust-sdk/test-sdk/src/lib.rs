
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn greet() -> *const u8 {
    let greeting = "Hello from Rust!\0";
    greeting.as_ptr()
}


pub fn hello() {
    println!("Hello, world!");
}

pub fn greet_native(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub mod math_utils {
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}