use std::fs;
use std::io;

fn main() {
    // Get status code from decompress() and exit with it
    std::process::exit(decompress());
}

fn decompress() -> i32 {
    // Get command line arguments as a vector of strings.
    let args: Vec<String> = std::env::args().collect();

    // If the amount of arguments supplied does not suffice, print CLI
    // usage message and return 1 to main().
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return 1;
    }

    // Get the first command line argument and assign it to the file_name var.
    // We should then open the file and assign it to the file var. If such a file
    // does not exist, the program will panic.
    let file_name = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&file_name).unwrap();

    // Create a new ZipArchive from the file var. If the file is not a valid
    // zip file, the program will panic.
    let mut archive = zip::ZipArchive::new(file).unwrap();

    // For each file in the zip archive, extract it to the current directory.
    for i in 0..archive.len() {
        // Set "file" equal to each file in the archive.
        // Achieve this by getting the archive_by_index,
        // and looping over every item in the archive.
        let mut file = archive.by_index(i).unwrap();

        // Set the output path of the current file to its enclosed name
        // if there is no name, continue.
        let output_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // If the file has a comment, print it.
        let comment = file.comment();
        if !comment.is_empty() {
            println!("File {} has the comment: {}", i, comment);
        }

        // If the file is a directory, create a new directory in the current location
        // with the same name.
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, output_path.display());
            fs::create_dir_all(&output_path).unwrap();

        // If the file is not a directory, create the file. If the file has a 
        // parent directory that does not already exist locally, create it.
        // We should also do some simple print statements to show what is happening.
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                output_path.display(),
                file.size()
            );
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(&parent).unwrap();
                }
            }
            let mut output_file = fs::File::create(&output_path).unwrap();
            io::copy(&mut file, &mut output_file).unwrap();
        }

        // Set file permissions for Unix systems.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&output_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    // Since we reach this point with no panics,
    // return with status code 0 (ok)
    return 0;
}
