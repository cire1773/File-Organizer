use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

fn get_category(extension: &str) -> &'static str {
    // The compiler optimizes this match statement heavily.
    match extension {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" => "Images",
        "mp4" | "mkv" | "mov" | "avi" => "Videos",
        "pdf" | "docx" | "txt" | "xlsx" | "pptx" | "xls" => "Documents",
        "zip" | "rar" | "7z" | "tar" | "gz" => "Archives",
        "py" | "js" | "cpp" | "rs" | "html" | "css" => "Scripts",
        "exe" | "msi" | "bat" | "sh" => "Executables",
        _ => "Others",
    }
}

fn get_unique_path(dest_folder: &Path, file_path: &Path) -> PathBuf {
    let filename = file_path.file_name().unwrap_or_default();
    let mut target_path = dest_folder.join(filename);

    if !target_path.exists() {
        return target_path;
    }

    let stem = file_path.file_stem().unwrap_or_default().to_string_lossy();
    let suffix = file_path
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();

    let mut counter = 1;
    loop {
        let new_name = format!("{}_{}{}", stem, counter, suffix);
        target_path = dest_folder.join(new_name);
        
        if !target_path.exists() {
            return target_path;
        }
        counter += 1;
    }
}

fn main() {
    // 1. Argument Parsing
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        process::exit(1);
    }

    let target_dir = Path::new(&args[1]);

    // Validation
    if !target_dir.exists() {
        eprintln!("Error: The directory '{}' does not exist.", target_dir.display());
        process::exit(1);
    }
    if !target_dir.is_dir() {
        eprintln!("Error: The path '{}' is not a directory.", target_dir.display());
        process::exit(1);
    }

    // Safety: Get the canonical path of the compiled Rust executable itself
    let current_exe = env::current_exe()
        .ok()
        .and_then(|p| p.canonicalize().ok());

    println!("Scanning: {} ...", target_dir.canonicalize().unwrap_or_else(|_| target_dir.to_path_buf()).display());

    let start_time = Instant::now();
    let mut files_moved = 0;

    // 2. Iterate through items in the directory
    if let Ok(entries) = fs::read_dir(target_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Safety: Skip directories
            if path.is_dir() {
                continue;
            }

            // Safety: Skip the executable itself if it's running from the target directory
            if let Some(ref exe_path) = current_exe {
                if let Ok(canon_path) = path.canonicalize() {
                    if canon_path == *exe_path {
                        continue;
                    }
                }
            }

            // 3. Determine Category
            let extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase(); // Case-insensitive matching

            let category = get_category(&extension);
            let dest_folder = target_dir.join(category);

            // Create destination folder if it doesn't exist
            if !dest_folder.exists() {
                if let Err(e) = fs::create_dir_all(&dest_folder) {
                    eprintln!("Failed to create directory {}: {}", dest_folder.display(), e);
                    continue;
                }
            }

            // 4. Handle Conflicts and Move
            let dest_path = get_unique_path(&dest_folder, &path);

            match fs::rename(&path, &dest_path) {
                Ok(_) => files_moved += 1,
                Err(e) => eprintln!("Error moving {}: {}", path.display(), e),
            }
        }
    } else {
        eprintln!("Error: Could not read directory '{}'.", target_dir.display());
        process::exit(1);
    }

    let elapsed_time = start_time.elapsed().as_secs_f64();

    println!("{:-<40}", "");
    println!("✅ Organized {} files in {:.4} seconds.", files_moved, elapsed_time);
}