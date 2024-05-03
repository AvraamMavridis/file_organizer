mod categories;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use crate::categories::CATEGORIES;

type DirEntries = Vec<fs::DirEntry>;
type Categories = HashMap<String, Vec<String>>;

fn list_files_in_directory(path: &Path) -> DirEntries {
    let mut files = vec![];
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                files.push(entry);
            }
        }
    }
    files
}

fn get_category(ext: &str) -> Option<&'static str> {
    CATEGORIES.get(ext).cloned()
}

fn categorize_files(files: DirEntries) -> HashMap<String, Vec<String>> {
    let mut categories: HashMap<String, Vec<String>> = HashMap::new();
    for file in files {
        let filetype: String = file
            .path()
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();
        let category = get_category(&filetype);
        if category.is_some() {
            categories
                .entry(category.unwrap().to_string())
                .or_insert_with(Vec::new)
                .push(file.path().to_string_lossy().into_owned());
        }
    }

    categories
}

fn move_files(path: &str, category_files: &[String]) {
    let new_dir_path = Path::new(&path);

    for file in category_files {
        let old_path = Path::new(file);
        let new_path = new_dir_path.join(old_path.file_name().unwrap());
        match fs::rename(&old_path, &new_path) {
            Ok(_) => println!("Successfully moved file to {}", new_path.display()),
            Err(e) => println!("Failed to move file: {}", e),
        }
    }
}

fn handle_category(path: &str, cat: &str, category_files: &[String]) {
    let new_dir_path = Path::new(path).join(cat);
    if !new_dir_path.exists() {
        if let Err(e) = fs::create_dir(&new_dir_path) {
            println!("Failed to create directory: {}, error: {}", cat, e);
            return;
        }
    }
    move_files(path, category_files);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a directory path");
        return;
    }
    let path = &args[1];
    let files = list_files_in_directory(Path::new(path));

    let categories: Categories = categorize_files(files);

    for (cat, files) in &categories {
        handle_category(path, &cat, files);
    }
}
