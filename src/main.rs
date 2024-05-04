mod categories;
mod files;

use crate::categories::get_category;
use crate::files::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

type Categories = HashMap<String, Vec<String>>;

/// Categorizes files based on their file type.
///
/// This function takes a list of directory entries and categorizes them based on their file type.
/// It creates a HashMap where the keys are the categories (file types) and the values are vectors
/// containing the paths of the files belonging to that category.
///
/// # Arguments
///
/// * `files` - A list of directory entries to be categorized.
///
/// # Returns
///
/// * A HashMap where the keys are the categories (file types) and the values are vectors
///   containing the paths of the files belonging to that category.
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

/// Handles the categorization of files.
///
/// This function takes a directory path, a category, and a list of files. It creates a new directory
/// for the category if it doesn't exist, and then moves the files into the new directory.
///
/// # Arguments
///
/// * `path` - The directory path where the new category directory will be created.
/// * `cat` - The category of the files.
/// * `category_files` - A list of files to be moved into the new category directory.
fn handle_category(path: &str, cat: &str, category_files: &[String]) {
    let new_dir_path = Path::new(path).join(cat);
    if !new_dir_path.exists() {
        if let Err(e) = fs::create_dir(&new_dir_path) {
            println!("Failed to create directory: {}, error: {}", cat, e);
            return;
        }
    }

    println!("{}", new_dir_path.to_str().unwrap());
    move_files(new_dir_path.to_str().unwrap(), category_files);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a directory path");
        return;
    }
    let path = &args[1];

    let files: Vec<fs::DirEntry>;

    if args.contains(&"-r".to_string()) {
        files = list_files_in_directories_recursively(Path::new(path));
    } else {
        files = list_files_in_directory(Path::new(path));
    }

    let categories: Categories = categorize_files(files);

    for (cat, files) in &categories {
        println!("{} {}", path, &cat);
        handle_category(path, &cat, files);
    }
}
