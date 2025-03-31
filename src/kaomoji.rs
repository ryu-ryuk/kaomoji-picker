//importing
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs, path::Path}; // file sys handling // working with file paths // TO store KEY-VALUE pairs

// Collection of Kaomojis organized by categories
//
// Stores kaomojis in a HashMap where each key is a category name
//
// and each Value of kaomojis belong to that category

/// Default initialization for categories HashMap
fn default_categories() -> HashMap<String, Vec<String>> {
    HashMap::new()
}

// The HashMap is initialized to an empty state
// when the KaomojiList is created
const EMBEDDED_DATA: &str = include_str!("resources/KAOMOJIS.json");

#[derive(Debug, Serialize, Deserialize)]
pub struct KaomojiList {
    #[serde(default = "default_categories")]
    pub categories: HashMap<String, Vec<String>>,
}

impl KaomojiList {
    pub fn load_embedded() -> Self {
        serde_json::from_str(EMBEDDED_DATA)
            .expect("Failed to parse embedded JSON data")
    }
    // Loading Kaomojis data from the JSON file
    //
    // # Arguments: "file_path" referencing the JSOn file
    //
    //# Returns:
    //  A KaomojiList populated with data from the JSON file,
    //  or an empty KaomojiList if the file is empty or not absent.
    pub fn load_from_json(file_path: &str) -> Self {
        if !Path::new(file_path).exists() {
            eprintln!("File not found: {}", file_path);
            return Self::empty();
        }

        match fs::read_to_string(file_path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_else(|e| {
                eprintln!("JSON parsing failed: {}", e);
                Self::empty()
            }),
            Err(e) => {
                eprintln!("File read error: {}", e);
                Self::empty()
            }
        }
    }

    /// Create empty KaomojiList instance
    fn empty() -> Self {
        KaomojiList {
            categories: default_categories()
        }
    }
}
