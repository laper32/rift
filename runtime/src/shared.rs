#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/js"]
#[include = "dist/**/*.js"]
pub struct RuntimeFiles;
