use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = since_the_epoch.as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // Set the feature flag for tests8.rs
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
