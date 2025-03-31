// use std::io::Write;
// use reqwest::blocking::get;
// use std::error::Error;
// use reqwest;
// use scraper::{Html, Selector};
// use std::fs;
//
//
// Importing KaomojiList struct from the local kaomoji module
use crate::kaomoji::KaomojiList;
// use serde_json;
// to execute commands
use std::process::Command;
use std::path::Path;
// Copying the selected text to system clipboard
//
// Attempts to use wl-copy (Wayland) first, then falls back to cliphist
// Prints an error message if both methods fail
//
// # Arguments : "text" - The text to copy to the clipboard
pub fn copy_to_clipboard(text: &str) {
    // Try using wl-clipboard (Wayland)
    if Command::new("wl-copy")
        .arg(text)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        return;
    }

    // Fallback to cliphist if wl-copy failed or isn't available
    if Command::new("cliphist")
        .arg("store")
        .stdin(std::process::Stdio::piped()) // setting up pipe for stdin
        .spawn() // spawning the process
        .and_then(|mut child| {
            // Write the text to the process's stdin
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write; // locally importing the write trait 
                stdin.write_all(text.as_bytes())?; // write txt bytes to stdin 
            }
            child.wait() // wait to complete
        })
        .map(|status| status.success()) // check if passed
        .unwrap_or(false)
    {
        return;
    }

    // If both fails, print an error message :o
    eprintln!("Failed to copy to clipboard. Ensure `wl-clipboard` or `cliphist` is installed.");
}

// Loads kaomoji data from a JSON file
//
// # Returns
// A KaomojiList containing kaomojis loaded from the JSON file

pub fn load_kaomojis() -> KaomojiList {
    let external_path = "/usr/share/kaomoji-picker/KAOMOJIS.json";
    

    // complications if i should let users provide custom files without recompiling the binary 
    if Path::new(external_path).exists() {
        KaomojiList::load_from_json(external_path)
    } else {
        eprintln!("Using embedded kaomojis.");
        KaomojiList::load_embedded()
    }
}


// trying to fetch the kaomojis from a online source so it can be updated
//
//
// pub fn load_or_fetch_kaomojis() -> KaomojiList {
//     let cache_file = "kaomoji_cache.json";

//     if let Ok(data) = fs::read_to_string(cache_file) {
//         if let Ok(kaomoji_list) = serde_json::from_str(&data) {
//             return kaomoji_list;
//         }
//     }
//     // let fetched_kaomojis = fetch_kaomojis();
//     // let kaomoji_list = KaomojiList {
//     //     favorites: vec![],
//     //     recent: vec![],
//     //     fetched: fetched_kaomojis,
//     //     custom: vec![],
//     // };
//     fs::write(cache_file, serde_json::to_string(&kaomoji_list).unwrap()).unwrap();
//     kaomoji_list
// }
// pub fn fetch_kaomojis() -> Vec<String> {
//     let url = "https://www.emojicombos.com/";
//     let response = reqwest::blocking::get(url).unwrap().text().unwrap();

//     let document = Html::parse_document(&response);
//     let selector = Selector::parse("span.kaomoji").unwrap();  // Adjust based on actual HTML structure

//     let kaomojis: Vec<String> = document
//         .select(&selector)
//         .map(|element| element.text().collect::<String>())
//         .collect();

//     kaomojis
// }
