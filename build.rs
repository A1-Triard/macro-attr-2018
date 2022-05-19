use rustc_version::{Version, version};

fn main() {
    let version = version().unwrap();
    assert!(version.major >= 1);
    if version >= Version::parse("1.61.0").unwrap() {
        println!("cargo:rustc-cfg=rustc_1_61");
    }
}
