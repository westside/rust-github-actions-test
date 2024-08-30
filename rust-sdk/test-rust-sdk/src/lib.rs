
pub fn hello() {
    println!("Hello, world!");
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub mod math_utils {
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}