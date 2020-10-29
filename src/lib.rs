mod utils;
use sha2::{Digest, Sha256, Sha512};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn fibonacci(input: u64) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..input {
        let t = a + b;
        a = b;
        b = t;
    }
    return b;
}

fn is_prime(num: u32) -> bool {
    let range: u32 = ((num as f64).sqrt() + 1.0) as u32;
    for i in 2..range {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

#[wasm_bindgen]
pub fn find_prime(num: u32) -> u32 {
    let mut cnt = 2;
    let mut idx = 3;
    let mut lastest_result = 3;
    while cnt < num {
        if is_prime(idx) == true {
            lastest_result = idx;
            cnt += 1;
        }
        idx += 2;
    }
    return lastest_result;
}

#[wasm_bindgen]
pub fn run_fibonacci(n: u64) -> u64 {
    return fibonacci(n);
}

#[wasm_bindgen]
pub fn sha256(input: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let string = format!("{:x}", result);
    println!("{}", string);
    string
}

#[wasm_bindgen]
pub fn sha512(input: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input);
    let result = hasher.finalize();
    let string = format!("{:x}", result);
    println!("{}", string);
    string
}
