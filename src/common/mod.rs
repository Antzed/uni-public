use std::path::PathBuf;
use dirs;
use std::fs;

pub fn expand_home_path(unexpanded_path: &String) -> PathBuf {
    let expand_home_path = if unexpanded_path.starts_with("~/") {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        home_dir.join(&unexpanded_path[2..])
    } else {
        PathBuf::from(unexpanded_path)
    };
    expand_home_path
}

pub fn get_db_path() -> PathBuf {
    let mut db_dir = dirs::home_dir().expect("Home directory not found");
    db_dir.push("app");
    db_dir.push("uni");   
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir).expect("Failed to create directory");
    }

    db_dir
}

pub fn get_uni_repo_path() -> PathBuf {
    let mut uni_repo_dir = dirs::home_dir().expect("Home directory not found");
    uni_repo_dir.push("code");
    uni_repo_dir.push("uni");   

    uni_repo_dir
}