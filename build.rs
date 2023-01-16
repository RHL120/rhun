fn main() {
    println!("cargo:rustc-link-lib=bsd");
    println!("cargo:rustc-link-lib=crypt");
}
