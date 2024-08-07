use crate::module::MODULE_DIR_NAME;
use homedir;

enum DirectoryLocation {
    Installation,
    UserProfile,
    Project,
}

// engine.dll一定和rift.exe在同一个目录下
impl DirectoryLocation {
    fn get_directory_location(&self) -> String {
        match self {
            DirectoryLocation::Installation => std::env::current_dir()
                .unwrap()
                .parent()
                .unwrap()
                .join(MODULE_DIR_NAME)
                .to_str()
                .unwrap()
                .to_owned(),
            DirectoryLocation::UserProfile => homedir::my_home()
                .unwrap()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            DirectoryLocation::Project => "project".to_owned(),
        }
    }
}

#[test]
fn test_dir_location() {
    // let dir = DirectoryLocation::Installation.get_directory_location();
    // assert_eq!(dir, "D:\\rust\\rift\\modules");
    // let dir = DirectoryLocation::UserProfile.get_directory_location();
    // assert_eq!(dir, "C:\\Users\\Administrator");
    // let dir = DirectoryLocation::Project.get_directory_location();
    // // assert_eq!(dir, "project");
}
