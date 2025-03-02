use std::fs;
use std::io;

fn main(){
    std::process::exit(decompress_logic());
}

fn decompress_logic() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return 1;
    }

    let filename = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&filename).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len(){
        let mut file = archive.by_index(i).unwrap();
        let output_path = match file.enclosed_name(){
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty(){
                println!("File {} comment: {}", i, comment);
            }
        }
        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, output_path.display());
            fs::create_dir_all(&output_path).unwrap();
        }else {
            println!("File {} extracted to \"{}\" ({} bytes)", i, output_path.display(), file.size());
            if let Some(p) = output_path.parent(){
                if !p.exists(){
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&output_path).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode(){
                fs::set_permissions(&output_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0
}