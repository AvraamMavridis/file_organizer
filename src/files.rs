use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub type DirEntries = Vec<fs::DirEntry>;

/// Lists all files in a given directory.
///
/// This function takes a directory path as an argument and returns a vector of directory entries
/// representing all the files in the directory. If the path is not a directory or an error occurs
/// while reading the directory, it returns an empty vector.
///
/// # Arguments
///
/// * `path` - The path of the directory to list files from.
///
/// # Returns
///
/// * A vector of directory entries representing all the files in the directory.
pub fn list_files_in_directory(path: &Path) -> DirEntries {
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

/// Lists all files in a given directory and its subdirectories recursively.
///
/// This function takes a directory path as an argument and returns a vector of directory entries
/// representing all the files in the directory and its subdirectories. If the path is not a directory
/// or an error occurs while reading the directory, it returns an empty vector.
///
/// # Arguments
///
/// * `path` - The path of the directory to list files from.
///
/// # Returns
///
/// * A vector of directory entries representing all the files in the directory and its subdirectories.
pub fn list_files_in_directories_recursively(path: &Path) -> DirEntries {
    let mut files = vec![];

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                println!("{}", entry.path().to_string_lossy());
                files.push(entry);
            }
        }
    }

    files
}

/// Moves files to a specified directory.
///
/// This function takes a directory path and a list of files as arguments. It moves each file in the list
/// to the specified directory. If a file cannot be moved for any reason, it prints an error message.
///
/// # Arguments
///
/// * `path` - The path of the directory where the files should be moved.
/// * `category_files` - A list of files to be moved.
pub fn move_files(path: &str, category_files: &[String]) {
    let new_dir_path = Path::new(&path);

    for file in category_files {
        let old_path = Path::new(file);
        let new_path = new_dir_path.join(old_path.file_name().unwrap());
        match fs::rename(&old_path, &new_path) {
            Ok(_) => println!("Successfully moved file from {} to {}", old_path.display(), new_path.display()),
            Err(e) => println!("Failed to move file: {}", e),
        }
    }
}
