#[allow(dead_code)]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
