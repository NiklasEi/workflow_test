use std::env;

fn main() {
    // do nothing, just check build time in workflow for macOS
    let _target = env::var("TARGET").unwrap();
}
