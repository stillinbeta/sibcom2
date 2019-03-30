fn main() {
    println!("cargo:rerun-if-changed=../site.yaml");
    println!("cargo:rerun-if-changed=../assets");
}
