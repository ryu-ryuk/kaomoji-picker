// importing
use std::{
    error::Error,
    fs,                        // file sys handling
    io::{Read, Write},         // I/O operations
    process::{Command, Stdio}, // to spawn Rofi
};

/// Takes the config object and a string list of options, and returns the selected index
/// ofc error if something is wrong
pub fn get_index(config: &crate::config::Config, list: &String) -> Result<usize, Box<dyn Error>> {
    // let me explain the flow:
    // Crafts a new Rofi command with the following arguments:
    // "-i": Return the index of the selected item
    // "-dmenu": Run in dmenu mode for text selection
    // "-p": Set the prompt text
    let mut rofi = Command::new(&config.rofi_binary)
        .arg("-format")
        .arg("i")
        .arg("-dmenu")
        .arg("-p")
        .arg(&config.prompt)
        .stdin(Stdio::piped()) // pipes for stdin
        .stdout(Stdio::piped()) // pipes for stdout
        .spawn()?; // spawning Rofi || error if fails

    // Write the list of options to the stdin of Rofi
    if let Some(mut stdin) = rofi.stdin.take() {
        stdin.write_all(list.as_bytes())?; // write the bytes of the list string
    } else {
        return Err("Failed to write to rofi stdin".into());
    }

    // Read the selected index from Rofi's stdout
    let mut index = String::new();
    if let Some(mut stdout) = rofi.stdout.take() {
        stdout.read_to_string(&mut index)?; // read the output into the index string
    } else {
        return Err("Failed to read rofi output".into());
    }

    // Parse the index string into usize and return it
    //
    index.trim().parse().map_err(|_| "Invalid selection".into())
}
// Runs Rofi with a grid layout for displaying options
// I didn't look up if there are better ways to do this (there must be)
// But this idea is taken from my wallpaper changing script | i'll reference it later ;)
//
// Takes a config obj and a str of options, returns the selected option as a string
// None if no selection or errors :(
pub fn run_rofi_with_grid(config: &crate::config::Config, options: &str) -> Option<String> {
    // Creates a temporary theme file for the grid layout
    let temp_theme_path = "/tmp/rofi_kaomoji_grid.rasi";

    // the themeing for rofi popup
    let temp_theme_content = r#"
    * {
        lines: 6;
    }
    window {
        width: 70%;
        height: 30%;
        location: center;
    }
    listview {
        columns: 4;
        spacing: 10px;
    }
    element {
        padding: 5px;
        orientation: vertical;
    }
    element-icon {
        size: 0px;
        border-radius: 2px;
    }
    element-text {
        enabled: true;
        padding-top: 5px;
        font-size: 20px;
        horizontal-align: 0.5;
    }
    "#;

    // Writes the theme content to the temporary file
    // Return early with None if this fails
    fs::write(temp_theme_path, temp_theme_content).expect("Failed to write temporary Rofi theme");

    // Prepare the Rofi command with the custom theming
    let mut rofi = Command::new(&config.rofi_binary);
    rofi.arg("-dmenu")
        .arg("-p")
        .arg(&config.prompt)
        .arg("-theme-str")
        .arg(format!("@import \"{}\"", temp_theme_path))
        .arg("-show-icons")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());

    // Spawning the Rofi process | None if fails
    let mut child = rofi.spawn().ok()?;

    // Writes the options to Rofi's stdin | converting errors to None if fails
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(options.as_bytes()).ok()?;
    }

    // Reading the selected option from Rofi's stdout
    let mut result = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        use std::io::Read; // Local importing Read trait
        stdout.read_to_string(&mut result).ok()?;
    }

    // Cleaning up the temporary theme file
    // Ignoring errors during cleanup
    fs::remove_file(temp_theme_path).ok();

    // Returning the selected option | None if none selected
    let result = result.trim();
    if result.is_empty() {
        None
    } else {
        Some(result.to_string())
    }
}
