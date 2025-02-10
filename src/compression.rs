pub fn compress(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("No input files provided".to_string());
    }

    println!("Compressing files: {:?}", args);
    // Lógica de compresión...

    Ok(())
}
