mod cli;
mod hashing;
use log::{error, info};

fn main() {
    // Initialize the logger
    env_logger::init();

    // Parse command line arguments to get the file path
    let file_path = cli::parse_args();

    // Log the file path
    info!("File path: {}", file_path);

    let hash = match hashing::calculate_sha256(&file_path) {
        Ok(hash) => hash,
        Err(e) => {
            // If an error occurs, log the error and exit the program
            error!(
                "Error calculating SHA-256 hash for file '{}': {}",
                file_path, e
            );
            std::process::exit(1); // Exit with a non-zero status code to indicate an error
        }
    };

    info!("SHA-256 hash: {}", hash);
}
