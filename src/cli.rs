use clap::{Arg, Command};
use log::{info, warn};

/// Parses command line arguments and initializes the logger.
/// Returns the path to the file as a String.
pub fn parse_args() -> String {
    let matches = Command::new("Visual Hashing")
        .version("1.0")
        .author("Sebastian Beyer sebastian.beyer@live.com>")
        .about("Does awesome things")
        .arg(
            Arg::new("FILE")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Safe to call unwrap because FILE is required
    let file_path = matches.get_one::<String>("FILE").unwrap().to_string();

    info!("Starting application");
    if !file_path.is_empty() {
        info!("Using input file: {}", file_path);
    } else {
        // Log a warning if, for some reason, the file path is empty.
        warn!("No input file provided");
    }

    file_path
}
