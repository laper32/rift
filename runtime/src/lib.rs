#[derive(rust_embed::RustEmbed)]
#[folder = "js"]
#[include = "dist/**/*.js"]
pub struct RuntimeFiles;
