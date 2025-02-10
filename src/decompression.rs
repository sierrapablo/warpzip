pub fn decompress(input: &str, output: Option<String>, list: bool) -> Result<(), String> {
    if list {
        println!("Listing contents of {}", input);
    } else {
        println!("Extracting {} to {:?}", input, output);
    }
    Ok(())
}
