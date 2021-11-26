use std::path::PathBuf;

pub fn get_info(path: PathBuf) -> String {

    let file_ext = path.extension();
    let file_ext = match file_ext {   
        Some(ext) => ext.to_str().unwrap(),
        None => "none",
    };    

    match &*file_ext {

        // Programming language
        "rs" => {
            "Rust source code file.".into()
        }
        "go" => {
            "Go source code file.".into()
        }
        "sh" => {
            "Bash shell script file.".into()
        }

        // Media file
        "png" => {
            "Portable Network Graphics image file.".into()
        }
        "jpeg" => {
            "Lossy compressed digital image file.".into()
        }

        "none" => {
            "Bash shell script file.".into()
        }

        _ => {
            "Unknown file.".into()
        }
    }

}

