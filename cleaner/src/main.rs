use std::io;
use std::fs;

fn scan_system(){
    let dir_path = "/tmp/";
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                let file_path = format!("/tmp/{}", file_name_str);
                match fs::metadata(&file_path) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            
                        } else {
                            if let Err(err) = fs::remove_file(&file_path) {
                                eprintln!("Error removing file: {}", err);
                            } else {
                                println!("File removed successfully");
                            }
                        }
                    }
                    Err(_) => {
                        println!("An error occurred while checking the path.");
                    }
                }
            }
        }
    } else {
        eprintln!("Error reading directory");
    }
}

fn main() {
    println!("Would you like to clean your system? (Yes/no)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_lowercase();
    if input == "yes"{
        println!("Scanning system...");
        scan_system();
    }
}
