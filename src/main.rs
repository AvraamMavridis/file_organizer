use std::collections::HashMap;
use std::fs;
use std::env;
use std::fs::DirEntry;
use std::path::Path;

type DirEntries = Vec<fs::DirEntry>;

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

fn categorize_files(files: DirEntries) -> HashMap<String, DirEntries> {
    let mut categories = HashMap::new();
    for file in files {
        let filetype: String = file.path().extension().unwrap_or_default().to_string_lossy().to_lowercase();
        if filetype != "" {
            categories.entry(filetype).or_insert_with(Vec::new).push(file);
        }
    }

    categories
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a directory path");
        return;
    }
    let path = Path::new(&args[1]);
    let files = list_files_in_directory(path);

    let categories = categorize_files(files);

    for cat in categories {
        println!("{}", cat.0)
    }
}
