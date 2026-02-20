use crate::schema::{
    real::{TomlProject, TomlTarget},
    rift::TomlPlugin,
    r#virtual::{TomlFolder, TomlWorkspace},
};

mod real;
mod rift;
mod r#virtual;

pub struct TomlManifest {
    pub target: Option<TomlTarget>,
    pub project: Option<TomlProject>,
    pub folder: Option<TomlFolder>,
    pub workspace: Option<TomlWorkspace>,
    pub plugin: Option<TomlPlugin>,
}
