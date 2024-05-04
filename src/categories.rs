use once_cell::sync::Lazy;
use std::collections::HashMap;

static CATEGORIES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut categories = HashMap::new();
    categories.insert("mp4", "videos");
    categories.insert("avi", "videos");
    categories.insert("mov", "videos");
    categories.insert("wmv", "videos");
    categories.insert("mkv", "videos");
    categories.insert("flv", "videos");
    categories.insert("vob", "videos");
    categories.insert("ogv", "videos");
    categories.insert("3gp", "videos");
    categories.insert("m4v", "videos");
    categories.insert("webm", "videos");
    categories.insert("mpg", "videos");
    categories.insert("mpeg", "videos");
    categories.insert("txt", "documents");
    categories.insert("doc", "documents");
    categories.insert("docx", "documents");
    categories.insert("pdf", "documents");
    categories.insert("rtf", "documents");
    categories.insert("odt", "documents");
    categories.insert("odp", "documents");
    categories.insert("epub", "documents");
    categories.insert("md", "documents");

    categories.insert("bmp", "images");
    categories.insert("jpg", "images");
    categories.insert("jpeg", "images");
    categories.insert("png", "images");
    categories.insert("gif", "images");
    categories.insert("webp", "images");
    categories.insert("tiff", "images");
    categories.insert("ico", "images");
    categories.insert("raw", "images");
    categories.insert("svg", "images");
    categories.insert("heif", "images");
    categories.insert("heic", "images");
    categories.insert("psd", "images");

    categories.insert("xlsx", "sheets");
    categories.insert("csv", "sheets");
    categories.insert("xls", "sheets");
    categories.insert("ods", "sheets");
    categories.insert("xlsm", "sheets");
    categories.insert("xlsb", "sheets");

    categories.insert("ppt", "presentations");
    categories.insert("pptx", "presentations");
    categories
});

/// Returns the category of a given file extension.
///
/// This function takes a file extension as an argument and returns the corresponding category
/// as a static string. If the extension is not found in the predefined categories, it returns None.
///
/// # Arguments
///
/// * `ext` - The file extension for which the category is to be found.
///
/// # Returns
///
/// * An Option containing the category as a static string if found, or None if not found.
pub fn get_category(ext: &str) -> Option<&'static str> {
    CATEGORIES.get(ext).cloned()
}
