use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() -> io::Result<()> {
    // Part 1: Sequential Processing
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <ABSOLUTE_PATH_TO_FILE_DIRECTORY>");
        std::process::exit(1);
    }

    let path = &args[1];
    
    // Sequential Processing
    println!("Sequential Processing:");
    let sequential_word_count = process_files_sequentially(path)?;
    println!("Sequential Word Count: {}", sequential_word_count);

    // Concurrent Processing
    println!("\nConcurrent Processing:");
    let concurrent_word_count = process_files_concurrently(path)?;
    println!("Concurrent Word Count: {}", concurrent_word_count);

    Ok(())
}

fn scan_path(path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let mut txt_files = Vec::new();
    let mut total_files = 0;
    let mut txt_count = 0;

    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        total_files += 1;

        if let Some(ext) = file_path.extension() {
            if ext == "txt" {
                txt_files.push(file_path);
                txt_count += 1;
            }
        }
    }

    println!("Found {} files in the directory and {} files we can process", total_files, txt_count);

    if txt_count == 0 {
        eprintln!("No .txt files found to process");
        std::process::exit(1);
    }

    Ok(txt_files)
}

fn read_file(path: &Path) -> io::Result<Vec<String>> {
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error reading file {:?}, skipping it. Error: {}", path, e);
            return Err(e);
        }
    };

    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn calculate_file(lines: &[String]) -> usize {
    // Optional bonus: Simulating a delay for concurrency demonstration
    // thread::sleep(Duration::from_secs(1));

    lines.iter()
        .flat_map(|line| line.split_whitespace())
        .count()
}

fn process_files_sequentially(path: &str) -> io::Result<usize> {
    let files = scan_path(path)?;
    let mut total_word_count = 0;

    for file_path in files {
        match read_file(&file_path) {
            Ok(lines) => {
                let word_count = calculate_file(&lines);
                total_word_count += word_count;
            },
            Err(_) => continue,
        }
    }

    Ok(total_word_count)
}

fn process_files_concurrently(path: &str) -> io::Result<usize> {
    let files = scan_path(path)?;
    let total_word_count = Arc::new(Mutex::new(0));

    let handles: Vec<_> = files.into_iter().map(|file_path| {
        let count_clone = Arc::clone(&total_word_count);
        thread::spawn(move || {
            match read_file(&file_path) {
                Ok(lines) => {
                    let word_count = calculate_file(&lines);
                    let mut total = count_clone.lock().unwrap();
                    *total += word_count;
                },
                Err(_) => (),
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = *total_word_count.lock().unwrap();
    Ok(final_count)
}