fn main() {
    let prefix = if cfg!(feature = "github-pages") {
        "yewlish-neo/"
    } else {
        "/"
    };

    println!("cargo:warning=LINK_PREFIX is: {}", prefix);
    println!("cargo:rustc-env=LINK_PREFIX={}", prefix);
}
