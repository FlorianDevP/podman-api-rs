use std::path::PathBuf;

pub mod all;
pub mod clippy;
pub mod codegen;
pub mod doc;
pub mod fmt;
pub mod lint;
pub mod test;

pub(super) fn cargo() -> PathBuf {
    std::env::var("CARGO")
        .map(PathBuf::from)
        .ok()
        .unwrap_or_else(|| PathBuf::from("cargo"))
}
