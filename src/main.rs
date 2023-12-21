use std::env;
use std::fs;
use std::path::PathBuf;
use std::error::Error;

#[derive(Debug)]
struct FileType {
    content: String,
    file_type: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let contract_path = args.iter().find(|arg| arg.starts_with("--contract")).unwrap_or(&"".to_string());
    let type_path = args.iter().find(|arg| arg.starts_with("--type")).unwrap_or(&"".to_string());

    let contract_files = recursive_read_file(PathBuf::from(contract_path))?;
    let type_files = recursive_read_file(PathBuf::from(type_path))?;

    for file in contract_files.iter().chain(type_files.iter()) {
        println!("{:?}", file);
    }

    Ok(())
}

fn get_file_content(file_path: PathBuf) -> Result<FileType, Box<dyn Error>> {
    let content = fs::read_to_string(file_path.clone())?;
    let file_type = if file_path.extension().unwrap_or_default() == "ts" {
        "Typescript Type"
    } else {
        "API Contract"
    };

    Ok(FileType { content, file_type })
}

fn recursive_read_file(dir_path: PathBuf) -> Result<Vec<FileType>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(recursive_read_file(path)?);
        } else {
            files.push(get_file_content(path)?);
        }
    }
    Ok(files)
}
