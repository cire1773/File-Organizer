import os
import sys
import shutil
import time
import argparse
from pathlib import Path

# --- Configuration ---
FILE_CATEGORIES = {
    "Images": {".jpg", ".jpeg", ".png", ".gif", ".bmp"},
    "Videos": {".mp4", ".mkv", ".mov", ".avi"},
    "Documents": {".pdf", ".docx", ".txt", ".xlsx", ".pptx", ".xls"},
    "Archives": {".zip", ".rar", ".7z", ".tar", ".gz"},
    "Scripts": {".py", ".js", ".cpp", ".rs", ".html", ".css"},
    "Executables": {".exe", ".msi", ".bat", ".sh"},
}

def get_category(file_path):
    """Determines the category based on file extension."""
    ext = file_path.suffix.lower()
    for category, extensions in FILE_CATEGORIES.items():
        if ext in extensions:
            return category
    return "Others"

def get_unique_path(dest_folder, filename):
    """
    Generates a unique path if the file already exists.
    Appends _1, _2, etc. to the filename.
    """
    file_path = dest_folder / filename
    if not file_path.exists():
        return file_path

    stem = file_path.stem
    suffix = file_path.suffix
    counter = 1

    while file_path.exists():
        new_name = f"{stem}_{counter}{suffix}"
        file_path = dest_folder / new_name
        counter += 1
    
    return file_path

def organize_directory(target_dir):
    target_path = Path(target_dir)

    # validation
    if not target_path.exists():
        print(f"Error: The directory '{target_dir}' does not exist.")
        sys.exit(1)
    if not target_path.is_dir():
        print(f"Error: The path '{target_dir}' is not a directory.")
        sys.exit(1)

    # Safety: Get the absolute path of the script itself to prevent moving it
    script_path = Path(__file__).resolve()
    
    start_time = time.time()
    files_moved = 0

    print(f"Scanning: {target_path.resolve()} ...")

    # Iterate through items in the directory
    for item in target_path.iterdir():
        # Safety: Skip directories and the script itself
        if item.is_dir():
            continue
        if item.resolve() == script_path:
            continue
        
        # Determine Category
        category = get_category(item)
        dest_folder = target_path / category
        
        # Create destination folder if it doesn't exist
        dest_folder.mkdir(exist_ok=True)
        
        # specific conflict handling (image.jpg -> image_1.jpg)
        dest_path = get_unique_path(dest_folder, item.name)
        
        try:
            shutil.move(str(item), str(dest_path))
            files_moved += 1
            # Optional: Print verbose operations
            # print(f"Moved: {item.name} -> {category}/")
        except Exception as e:
            print(f"Error moving {item.name}: {e}")

    end_time = time.time()
    elapsed_time = end_time - start_time

    print("-" * 40)
    print(f"✅ Organized {files_moved} files in {elapsed_time:.4f} seconds.")

if __name__ == "__main__":
    # Argument parsing
    parser = argparse.ArgumentParser(description="Organize files in a directory by extension.")
    parser.add_argument("directory", help="The path to the directory you want to organize.")
    
    args = parser.parse_args()
    
    organize_directory(args.directory)