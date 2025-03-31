// imports from the current crate
mod config; // Configuration management
mod kaomoji;
mod rofi; // Import Rofi functions
mod utils; // Utility functions (clipboard, file handling) // Kaomoji data structure

// Importing specific funcs
use crate::config::Config; // Configuration struct
use crate::rofi::{get_index, run_rofi_with_grid}; // Rofi interface functions
use crate::utils::{copy_to_clipboard, load_kaomojis}; // Utility functions

///
///
fn main() {
    // Loading configuration
    let config = Config::load();

    // Loads kaomojis from the JSON file
    let kaomojis_db = load_kaomojis();
    // Extract the category names from the kaomoji database
    // Cloning each key in order to create owned String values in the vector
    let categories: Vec<String> = kaomojis_db.categories.keys().cloned().collect();

    // Checking if there are categories are available
    if categories.is_empty() {
        eprintln!("No kaomoji categories found!");
        return;
    }

    // Displaying Rofi menu for category selection
    // Joining categories with newlines to create a list for Rofi
    if let Ok(index) = get_index(&config, &categories.join("\n")) {
        // Get the selected category name using the returned index
        let selected_category = &categories[index];

        // Checking if the selected category exists in the database
        // (This is redundant since we got the category from the keys,
        // but provides a safety check and better code structure)
        if let Some(_kaomoji_list) = kaomojis_db.categories.get(selected_category) {
            // Display rofi grid menu with kaomojis from the selected category
            //
            // Joining the kaomojis with newlines to create a list for Rofi
            if let Some(selected_kaomoji) = run_rofi_with_grid(
                &config,
                &kaomojis_db.categories[selected_category].join("\n"),
            ) {
                // Copy the selected kaomoji to the clipboard
                copy_to_clipboard(&selected_kaomoji);
                // Printing confirmation ;)
                println!("Copied to clipboard: {}", selected_kaomoji);
            }
        } else {
            eprintln!("No kaomojis found for category: {}", selected_category);
        }
    }
}
