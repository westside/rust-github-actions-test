use test_sdk::{greet_native, hello};
use test_sdk::math_utils::multiply;

fn main() {
    hello();
    let msg = greet_native("hello");

    println!("msg: {}", msg);
    let res = multiply(43, 44);

    println!("43 + 44 = {}", res);  // Outputs: "5 + 3 = 8"
}
