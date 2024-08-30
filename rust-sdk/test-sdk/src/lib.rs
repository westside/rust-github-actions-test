
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    test_rust_sdk::math_utils::add(a, b)
}

#[no_mangle]
pub extern "C" fn greet() -> *const u8 {
    let greeting = "Hello from Rust!\0";
    greeting.as_ptr()
}