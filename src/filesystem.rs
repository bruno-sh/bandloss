use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::Path,
};

fn create_directory(path: &Path) {
    if let Err(err) = create_dir_all(path) {
        eprintln!(
            "error: failed to create \"{}\" ({:?})",
            path.display(),
            err.kind()
        );
        std::process::exit(1);
    }
}

pub fn create_file(bytes: bytes::Bytes, name: &str) {
    let documents_path = dirs::document_dir().unwrap_or_else(|| {
        eprintln!("error: failed to access the documents directory.");
        std::process::exit(1);
    });

    let bandloss_path = documents_path.join("bandloss");
    create_directory(&bandloss_path);

    let file_path = bandloss_path.join(format!("{}.mp3", name));

    let mut file = match File::create(&file_path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!(
                "error: failed to create \"{}\" ({:?})",
                file_path.display(),
                err.kind()
            );
            std::process::exit(1);
        }
    };

    if let Err(err) = file.write_all(&bytes) {
        eprintln!(
            "error: failed to write bytes to \"{}\" ({:?})",
            file_path.display(),
            err.kind()
        );
        std::process::exit(1);
    }
}
