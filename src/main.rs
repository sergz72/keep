use std::collections::HashMap;
use std::env::args;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::exit;

struct File {
    path: PathBuf,
    date: std::time::SystemTime
}

fn main() -> Result<(), Error> {
    let arguments: Vec<String> = args().skip(1).collect();
    if arguments.len() < 2 {
        println!("Usage: keep folder_name number_of_latest_files_to_keep [dry_run]");
        exit(1);
    }
    let folder_name = arguments[0].clone();
    let number_of_files = arguments[1].parse::<usize>()
        .map_err(|e|Error::new(ErrorKind::Other, e.to_string()))?;
    let dry_run = arguments.len() == 3 && arguments[2] == "dry_run";

    println!("Keeping {} files in folder {}{}", number_of_files, folder_name, if dry_run { " (dry run)" } else { "" });

    let mut names = HashMap::new();
    let folder_path = Path::new(&folder_name);
    for result in std::fs::read_dir(folder_path)? {
        let r = result?;
        let path = r.path();
        if !path.is_file() {
            continue
        }
        if let Some(name) = path.file_stem() {
            if let Some(file_name) = name.to_str() {
                let date = r.metadata()?.modified()?;
                let files = names.entry(file_name.to_string()).or_insert(Vec::new());
                files.push(File { path, date });
            }
        }
    }

    for (name, files) in names.iter_mut() {
        println!("Found {} files with name {}", files.len(), name);
        if files.len() <= number_of_files {
            continue;
        }
        files.sort_by(|a, b| b.date.cmp(&a.date));
        for file in files.iter().skip(number_of_files) {
            println!("Deleting {}", file.path.display());
            if !dry_run {
                std::fs::remove_file(&file.path)?;
            }
        }
    }

    Ok(())
}
