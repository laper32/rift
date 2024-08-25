pub mod evaluate;
pub mod loader;
mod script;
pub mod specifier;

// RA会在需要展开的时候变成弱智
// 所以需要非常小心的用
#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../runtime"]
#[include = "dist/**/*.js"]
pub struct RuntimeFiles;

pub fn init() {}

pub fn shutdown() {}
