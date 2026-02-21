use crate::schema::{
    real::{TomlProject, TomlTarget},
    rift::TomlPlugin,
    r#virtual::{TomlFolder, TomlWorkspace},
};
use serde::Deserialize;

pub mod real;
pub mod rift;
pub mod r#virtual;

#[derive(Debug, Deserialize)]
pub struct TomlManifest {
    pub target: Option<TomlTarget>,
    pub project: Option<TomlProject>,
    pub folder: Option<TomlFolder>,
    pub workspace: Option<TomlWorkspace>,
    pub plugin: Option<TomlPlugin>,
}
