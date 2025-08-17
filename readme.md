# Copy Without Folder

A simple command-line tool for copying files from source directory to destination directory with the ability to exclude unwanted folders. The program guides you through the process step by step with an interactive interface.

🌍 **[Vietnamese Version](readme.vi.md)**

## Installation

```bash
# Clone repository  
git clone <repository-url>
cd copy-without-folder

# Build project
cargo build --release
```

## Usage

### Run the program

```bash
cargo run
```

The program will prompt you to enter:

1. **Source folder**: Path to the directory containing files to copy
2. **Destination folder**: Path to where you want to save the copied files  
3. **Exclude folders**: List of folders you don't want to copy (comma-separated)

### Real-world usage example

#### When running the program:

```
Enter source folder: /home/user/my-project
Enter destination folder: /home/user/backup
Enter folder names to exclude (comma-separated): node_modules, .git, target
🔍 Scanning...
📦 Total files to copy: 1250
🚫 Exclude folders: ["node_modules", ".git", "target"]
⚠️ Total excluded: 15000
Proceed with copy? (y/n): y
```

Then the program will display a progress bar and proceed with copying.

## Features

- ✅ User-friendly interactive interface
- ✅ Scan and count files before copying
- ✅ Copy all files from source directory
- ✅ Automatically create directory structure at destination
- ✅ Exclude multiple folders at once (comma-separated)
- ✅ Progress bar with emoji and detailed statistics
- ✅ Detailed error handling and notifications
- ✅ Confirmation prompt before copying

## How it works

1. **Phase 1 Scan**: The program will traverse the entire source directory to:
   - Count total files to be copied
   - Identify number of excluded files/folders
   - Display detailed statistics

2. **Confirmation**: Ask user for confirmation before proceeding with copy

3. **Execute Copy**: Copy each file with progress bar showing progress

## Folder Exclusion

The program will exclude:
- All folders with names matching the exclusion list
- All files and subfolders within excluded folders
- Search by exact name match (case-sensitive)

### Exclusion Examples

If you enter: `node_modules, .git, target, build`

The program will skip:
- `./node_modules/` and all contents inside
- `./frontend/node_modules/` and all contents inside  
- `./.git/` and all contents inside
- `./rust-project/target/` and all contents inside
- `./cpp-project/build/` and all contents inside

## Important Notes

1. **Source directory must exist**: Program will error if source directory is not found.

2. **Destination directory created automatically**: If destination directory doesn't exist, it will be created automatically when needed.

3. **Exclude by exact name match**: Program will exclude all folders with names exactly matching the entered list.

4. **File overwriting**: Existing files in destination directory will be overwritten without asking.

5. **Performance**: Program scans twice - first to count, second to copy. This ensures accurate progress bar.

## Common Use Cases

### Backup web project (exclude node_modules)
```
Enter source folder: ./my-web-project
Enter destination folder: ./backup/web-project-backup
Enter folder names to exclude: node_modules, .git, dist, build
```

### Backup Rust project (exclude target)
```
Enter source folder: ./my-rust-project  
Enter destination folder: ./backup/rust-project-backup
Enter folder names to exclude: target, .git
```

### Backup multi-language project
```
Enter source folder: ./monorepo-project
Enter destination folder: ./backup/monorepo-backup
Enter folder names to exclude: node_modules, target, build, dist, .git, __pycache__, venv
```

### Copy media folder (no exclusions)
```
Enter source folder: ./photos-2024
Enter destination folder: ./backup/photos-2024
Enter folder names to exclude: (leave empty, press Enter)
```

## Build and Run Executable

### Build release version
```bash
cargo build --release
```

### Run executable directly
```bash
./target/release/copy-without-folder
```

### Create alias for easy usage (optional)

Add to `~/.bashrc` or `~/.zshrc`:
```bash
alias copy-clean="/path/to/your/project/target/release/copy-without-folder"
```

Then you can run:
```bash
copy-clean
```

## Demo
```
🔍 Đang scan...

📦 Tổng số file sẽ copy: 19416
🚫 Exclude folders: [".git", "node_modules", "dist"]

Tiến hành copy? (y/n): y

⏳ Copying files...
█████████████████████████████████████████ 100%
✅ Hoàn tất!
```

## Technical Information

- **Language**: Rust
- **Main Dependencies**:
  - `walkdir`: Recursive directory traversal
  - `indicatif`: Progress bar and spinner
- **Platform**: Cross-platform (Windows, macOS, Linux)
- **Performance**: Optimized for copying large files with progress tracking

## Troubleshooting

### Permission denied error
```
❌ Cannot copy /path/to/file: Permission denied (os error 13)
```
**Solution**: Check read permissions for source directory and write permissions for destination directory.

### Source directory does not exist
Program will error immediately when starting scan. Double-check the entered path.

### Disk space full
```
❌ Cannot copy /path/to/file: No space left on device (os error 28)
```
**Solution**: Free up disk space or choose a different destination directory.

## Contributing

1. Fork this project
2. Create feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to branch (`git push origin feature/AmazingFeature`)
5. Open Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.
