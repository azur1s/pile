use std::path::PathBuf;

pub fn get_info(path: PathBuf) -> String {

    let file_ext = path.extension();
    let file_ext = match file_ext {   
        Some(ext) => ext.to_str().unwrap(),
        None => "none",
    };    

    let info: &str = match &*file_ext {

        // Text file
        "rs" => "Rust source code file.",
        "go" => "Go source code file.",
        "sh" => "Bash shell script file.",
        "js" => "Javascript source code file.",
        "ts" => "Typescript source code file.",
        
        "md" => "Markdown file",
        "html" => "HyperText Markup Language file",
        "txt" => "Text file",

        // Media file
        "png" => "Portable Network Graphics image file.",
        "jpeg" => "Lossy compressed digital image file.",

        // TODO: maybe check the file's content eg. "LICENSE" file can be mistaken as native
        // executable
        "none" => "Native executable",

        _ => "Unknown file.",
    };

    return info.into();
}
