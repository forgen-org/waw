fn main() {
    println!("cargo:rerun-if-changed=components/add.wit");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
