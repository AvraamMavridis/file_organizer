use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static CATEGORIES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut categories = HashMap::new();
    categories.insert("mp4", "videos");
    categories.insert("avi", "videos");
    // ... add all other mappings ...
    categories
});
