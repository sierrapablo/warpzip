use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use tar::{Builder, Header};
use zip::write::FileOptions;
use zip::ZipWriter;

/// Compresses a directory or file into a .tar.gz archive.
pub fn compress_tar_gz(source: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tar_gz = File::create(output)?;
    let encoder = GzEncoder::new(tar_gz, Compression::best());
    let mut tar = Builder::new(encoder);

    let path = Path::new(source);
    if path.is_dir() {
        tar.append_dir_all("", path)?;
    } else {
        let mut file = File::open(path)?;
        let mut header = Header::new_gnu();
        header.set_size(file.metadata()?.len());
        tar.append_data(&mut header, path, &mut file)?;
    }

    Ok(())
}

/// Compresses a directory or file into a .zip archive.
pub fn compress_zip(source: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(source);
    let file = File::create(output)?;
    let writer = BufWriter::new(file);
    let mut zip = ZipWriter::new(writer);
    let options: FileOptions<()> =
        FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let mut file = File::open(&file_path)?;
            zip.start_file(file_name, options)?;
            std::io::copy(&mut file, &mut zip)?;
        }
    } else {
        let mut file = File::open(path)?;
        zip.start_file(path.file_name().unwrap().to_str().unwrap(), options)?;
        std::io::copy(&mut file, &mut zip)?;
    }

    zip.finish()?;
    Ok(())
}
