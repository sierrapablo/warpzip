pub fn compress(
    input: &str,
    _output: Option<String>,
    level: u8,
    force: bool,
) -> Result<(), String> {
    println!(
        "Compressing {} with level {} (force: {})",
        input, level, force
    );
    Ok(())
}
