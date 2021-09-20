use generators::generate;

fn main() {
    println!("cargo:rerun-if-changed=src/");

    generate("./src");
}