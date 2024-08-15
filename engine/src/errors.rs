pub type RiftResult<T> = anyhow::Result<T>;

// pub struct SimpleError {
//     message: String,
// }

// impl Debug for SimpleError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str(&self.message)
//     }
// }

// impl Display for SimpleError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str(&self.message)
//     }
// }

// impl std::error::Error for SimpleError {}

// impl SimpleError {
//     pub fn new(message: &str) -> Self {
//         Self {
//             message: message.to_string(),
//         }
//     }
// }
