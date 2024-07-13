use std::os::unix::fs::PermissionsExt;

fn main() {
    if std::env::args().len() < 2 {
        eprintln!("Usage: descompress PATH_TO_COMPRESSED_FILE");
        return;
    }

    if !std::path::Path::new(&std::env::args().nth(1).unwrap())
        .to_owned()
        .exists()
    {
        println!(
            "\"{}\" file doesn't exists!",
            std::env::args().nth(1).unwrap()
        );
        return;
    }

    let start = std::time::Instant::now();

    let src = std::env::args().nth(1).unwrap().to_owned();
    let src_path = std::path::Path::new(&src);

    let src_file = std::fs::File::open(&src_path).unwrap();

    let mut archives = zip::ZipArchive::new(src_file).unwrap();

    for i in 0..archives.len() {
        let mut file = archives.by_index(i).unwrap();

        // INFO: check if the path is safe to access
        let out_path = match file.enclosed_name() {
            Some(path) => path.clone(),
            None => continue,
        };

        let cmnt = file.comment();
        if !cmnt.is_empty() {
            println!("File {} comment: {}", file.name(), cmnt);
        }

        // INFO: if the out path is dir, create them
        if file.name().ends_with("/") {
            std::fs::create_dir_all(&out_path).unwrap();
            continue;
        }

        println!(
            "file {} extracted to \"{}\" ({} bytes)",
            file.name().split("/").last().unwrap(),
            out_path.display(),
            file.size()
        );

        // INFO: Check if the parent dir exists or not, if not, create it
        if let Some(p) = out_path.parent() {
            if !p.exists() {
                std::fs::create_dir_all(&p).unwrap();
            }
        }

        // INFO: decompress
        let mut out_file = std::fs::File::create(&out_path).unwrap();
        std::io::copy(&mut file, &mut out_file).unwrap();

        #[cfg(unix)]
        {
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&out_path, std::fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    println!("Elapsed time: {:?}", start.elapsed());
}
