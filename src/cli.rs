use crate::compression::compress;
use crate::decompression::decompress;
use crate::logging::setup_logging;
use std::env;

pub fn run_cli() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: warpzip <command> <options>".to_string());
    }

    setup_logging();

    match args[1].as_str() {
        "warp" => compress(&args[2..])?,
        "unwarp" => decompress(&args[2..])?,
        _ => return Err("Unknown command".to_string()),
    }

    Ok(())
}
