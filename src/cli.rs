use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "warpzip")]
#[command(
    about = "A high-speed compression and decompression tool",
    version = "1.0"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Defines the available subcommands: `warp` (compress) and `unwarp` (decompress)
#[derive(Subcommand)]
pub enum Commands {
    /// Compress files or directories
    Warp {
        /// Input file or directory to compress
        input: String,

        /// Output file name (optional)
        #[arg(short, long)]
        output: Option<String>,

        /// Compression level (1-9)
        #[arg(short = 'l', long, default_value = "5")]
        level: u8,

        /// Force overwrite existing files
        #[arg(short, long)]
        force: bool,

        /// Type of archive to create: tar.gz or zip
        #[arg(long, default_value = "tar.gz", value_parser = clap::builder::PossibleValuesParser::new(["tar.gz", "zip"]))]
        archive_type: String,
    },

    /// Decompress files
    Unwarp {
        /// Input compressed file
        input: String,

        /// Output directory (optional)
        #[arg(short, long)]
        output: Option<String>,

        /// List contents instead of extracting
        #[arg(long)]
        list: bool,
    },
}

/// Parses the CLI arguments and executes the corresponding command.
pub fn run_cli() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Warp {
            input,
            output,
            level,
            force,
            archive_type,
        } => {
            println!(
                "Compressing {} -> {:?} (level: {}, force: {}, archive_type: {})",
                input, output, level, force, archive_type
            );

            match archive_type.as_str() {
                "tar.gz" => {
                    let output = output.unwrap_or_else(|| format!("{}.tar.gz", input));
                    crate::compression::compress_tar_gz(&input, &output)
                        .map_err(|e| e.to_string())?;
                }
                "zip" => {
                    let output = output.unwrap_or_else(|| format!("{}.zip", input));
                    crate::compression::compress_zip(&input, &output).map_err(|e| e.to_string())?;
                }
                _ => return Err("Unsupported archive type".to_string()),
            }

            Ok(())
        }
        Commands::Unwarp {
            input,
            output,
            list,
        } => {
            if list {
                println!("Listing contents of {}", input);
            } else {
                println!("Extracting {} -> {:?}", input, output);
            }
            crate::decompression::decompress(&input, output, list)
        }
    }
}
