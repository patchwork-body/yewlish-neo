fn main() {
    if let Ok(prefix) = std::env::var("LINK_PREFIX") {
        println!("cargo:rustc-env=LINK_PREFIX={}", prefix);
    } else {
        println!("cargo:warning=LINK_PREFIX is not set, defaulting to /");
        println!("cargo:rustc-env=LINK_PREFIX=/");
    }
}
