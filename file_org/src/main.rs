use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> { // this function will return Result (it could be either sucesss in which it will just run or a fail which will return fail  )
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || args[1] != "organize" {
        eprintln!("Usage: cargo run -- organize <folder_path>");
        std::process::exit(1);
    }

    let folder_path= Path::new(&args[2]);
    if !folder_path.exists() || !folder_path.is_dir() { // initial work up upon error for improper input
        eprintln!("Error: '{}' is not a valid directory", folder_path.display());
        std::process::exit(1);
    }

    println!("Organizing files in: {}", folder_path.display()); // prints the beginning o the organising file procedure

    let categories = vec![
        ("images", vec!["jpg", "png", "gif", "bmp", "svg"]),
        ("documents", vec!["pdf", "doc", "txt", "md"]),
        ("videos", vec!["mp4", "avi", "mkv", "mov"]),
        ("audio", vec!["mp3", "wav", "flac"]),
        ("archives", vec!["zip", "rar", "tar", "gz"]),
    ];

    let mut organized_count = 0;
    let mut error_count = 0;

    for entry in fs::read_dir(folder_path)? {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => {
                error_count += 1;
                continue;
            }
        };
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let extension = path.extension().and_then(|e| e.to_str());

        let mut category = "others";
        if let Some(ext) = extension.as_deref() {
            for (folder, exts) in &categories {
                if exts.contains(&ext) {
                    category = folder;
                    break;
                }
            }
        }

        let target_dir = folder_path.join(category);
        if !target_dir.exists() {
            fs::create_dir(&target_dir)?;
            println!("Created directory: {}/", category);
        }

        let file_name = match path.file_name() {
            Some(name) => name,
            None => {
                error_count += 1;
                continue;
            }
        };

        let target_path = target_dir.join(file_name);
        match fs::rename(&path, &target_path) {
            Ok(_) => {
                println!("Moved {} → {}/{}", path.file_name().unwrap().to_string_lossy(),category,file_name.to_string_lossy());
                organized_count += 1;
            }
            Err(_) => {
                eprintln!("Error moving {:?}", path);
                error_count += 1;
            }
        }
    }

    println!("\nSummary: {} files organized, {} errors", organized_count, error_count);

    Ok(())
}
