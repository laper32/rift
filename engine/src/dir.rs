use homedir;

pub enum PathIdentity {
    Installation,
    UserProfile,
    Project,
}

const NON_INSTALLATION_PATH_NAME: &str = ".rift";

// engine.dll一定和rift.exe在同一个目录下
impl PathIdentity {
    pub fn get_rift_path(path_id: PathIdentity) -> String {
        let installation_path = std::env::current_exe() // ${installation_path}/bin/rift.exe
            .unwrap()
            .parent() // ${installation_path}/bin
            .unwrap()
            .parent() // ${installation_path}
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        match path_id {
            PathIdentity::Installation => installation_path,
            PathIdentity::UserProfile => homedir::my_home()
                .unwrap()
                .unwrap()
                .join(NON_INSTALLATION_PATH_NAME) // ${env:HOME}/.rift
                .to_str()
                .unwrap()
                .to_owned(),
            // 前两个没什么好说的，重点是Project。
            // Project的.rift能且只能在项目的根目录。
            // 如果这个项目有workspace，那么.rift就在workspace的根目录，以此类推（aka: Project, Target）。
            // 有一个例外：Folder，Folder不参与任何.rift的查找。
            PathIdentity::Project => "project".to_owned(),
        }
    }
}
