use std::path::{Path, PathBuf};

pub fn make_rand_temp_file_path(file_path: &Path) -> PathBuf {
    let rand = gen_rand_path_component();
    let extension = format!("{rand}.tmp");
    file_path.with_extension(extension)
}

fn gen_rand_path_component() -> String {
    (0..4).fold(String::new(), |mut output, _| {
        output.push_str(&format!("{:02x}", rand::random::<u8>()));
        output
    })
}
