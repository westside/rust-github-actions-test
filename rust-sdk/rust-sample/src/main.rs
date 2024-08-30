use test_rust_sdk::{hello};
use test_rust_sdk::math_utils::multiply;

fn main() {
    hello();

    let res = multiply(43, 44);

    println!("43 + 44 = {}", res);  // Outputs: "5 + 3 = 8"
}
