pub fn decompress(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("No input files provided".to_string());
    }

    println!("Decompressing files: {:?}", args);
    // Lógica de descompresión...

    Ok(())
}
