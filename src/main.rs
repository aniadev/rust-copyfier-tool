use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};

fn main() -> io::Result<()> {
    let mut source = String::new();
    let mut destination = String::new();
    let mut exclude_input = String::new();

    print!("Nhập folder source: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut source)?;
    let source = source.trim();

    print!("Nhập folder destination: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut destination)?;
    let destination = destination.trim();

    print!("Nhập tên folder cần loại trừ (phân cách bởi dấu ,): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut exclude_input)?;
    let exclude_folders: Vec<String> = exclude_input
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    println!("🔍 Đang scan...");

    let mut total_files = 0;
    let mut skipped_files: Vec<PathBuf> = Vec::new();

   
    for entry in WalkDir::new(source)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if entry.file_type().is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if exclude_folders.iter().any(|excl| excl == name) {
                    skipped_files.push(path.to_path_buf());
                    continue;
                }
            }
        } else {
            // kiểm tra cha của file có trong exclude không
            if path.ancestors().any(|ancestor| {
                ancestor.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| exclude_folders.iter().any(|excl| excl == n))
                    .unwrap_or(false)
            }) {
                skipped_files.push(path.to_path_buf());
                continue;
            }

            total_files += 1;
        }
    }

    println!("📦 Tổng số file sẽ copy: {}", total_files);
    println!("🚫 Exclude folders: {:?}", exclude_folders);
    println!("⚠️ Total excluded: {:?}", skipped_files.len());

    // if !skipped_files.is_empty() {
    //     println!("⚠️ Những file sẽ bị bỏ qua:");
    //     for file in &skipped_files {
    //         // println!("  - {}", file.display());
    //     }
    // }

    print!("Tiến hành copy? (y/n): ");
    io::stdout().flush()?;
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;
    if confirm.trim().to_lowercase() != "y" {
        println!("❌ Đã hủy.");
        return Ok(());
    }

    // Progress bar setup
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );

    // tiến hành copy
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let mut skip = false;
            for excl in &exclude_folders {
                if entry.path().components().any(|c| c.as_os_str() == std::ffi::OsStr::new(excl)) {
                    skip = true;
                    break;
                }
            }
            if skip {
                continue;
            }

            let relative = entry.path().strip_prefix(source).unwrap();
            let dest_path = Path::new(destination).join(relative);

            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            if let Err(e) = fs::copy(entry.path(), &dest_path) {
                eprintln!("❌ Không thể copy {}: {}", entry.path().display(), e);
            }
            pb.inc(1);
        }
    }
    pb.finish_with_message("Copy hoàn tất!");

    Ok(())
}
