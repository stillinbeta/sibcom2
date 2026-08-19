fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=../assets/style.css");
    println!("cargo::rerun-if-changed=../assets/script.js");
    println!("cargo::rerun-if-changed=build.rs")
}
