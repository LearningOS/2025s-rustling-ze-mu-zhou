use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-cfg=feature=\"pass\"");
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
}