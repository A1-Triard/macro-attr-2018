use rustc_version::{Version, version};
use semver::{BuildMetadata, Prerelease};

fn main() {
    let mut version = version().unwrap();
    assert!(version.major >= 1);
    version.pre = Prerelease::EMPTY;
    version.build = BuildMetadata::EMPTY;
    if version >= Version::parse("1.62.0").unwrap() {
        println!("cargo:rustc-cfg=rustc_1_62");
    }
}
