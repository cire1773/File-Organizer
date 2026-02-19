# ⚡ File Organizer Benchmark: Python vs. Rust

A performance comparison between an interpreted language (Python) and a compiled systems language (Rust) for a real-world I/O bound task: organizing a cluttered directory by file extensions.

## 🎯 The Experiment
The goal of this project is to measure the execution speed, developer experience, and resource efficiency of writing a CLI automation tool in two very different languages. 

Both scripts perform the exact same logic:
1. Scan a target directory.
2. Categorize files based on their extension (Images, Videos, Documents, Archives, Scripts, Executables, Others).
3. Move the files into their respective subfolders.
4. Intelligently handle file name collisions (e.g., renaming `data.txt` to `data_1.txt` if it already exists).
5. Measure and report the total execution time.

## 📂 Repository Structure
```text
📦 file-organizer-benchmark
 ┣ 📂 python-version
 ┃ ┗ 📜 organize.py
 ┣ 📂 rust-version
 ┃ ┣ 📂 src
 ┃ ┃ ┗ 📜 main.rs
 ┃ ┗ 📜 Cargo.toml
 ┗ 📜 README.md
```

## 🚀 How to Run

### Python Version
Requires Python 3.x. No external dependencies needed.
```bash
cd python-version
python organize.py "/path/to/your/messy/folder"
```

### Rust Version
Requires Cargo and Rust `rustc`. 
*Note: For accurate benchmarking, always build using the `--release` flag!*
```bash
cd rust-version
cargo run --release -- "/path/to/your/messy/folder"
```

## 📊 Benchmark Results

*Tested on a directory containing roughly 10.000 files.*

| Metric | Python 3 | Rust (Release Build) |
| :--- | :--- | :--- |
| **Execution Time** | `112.953 seconds` | `43.036 seconds` |
| **Binary / Script Size** | `~ 4 KB` | `~ 25 MB` |
| **Memory Safety** | Handled by Garbage Collector | Guaranteed by Rust Compiler |

## 🧠 Key Takeaways
* **Python:** Incredibly fast to prototype and write. String manipulation and OS path handling are highly intuitive out of the box.
* **Rust:** Required more upfront design (handling `Result` types, ownership, and Borrow Checker rules for path manipulation), but the execution speed and memory safety guarantees are unmatched.
