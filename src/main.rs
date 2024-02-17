mod cli;
use log::{info, warn};

fn main() {
    // Initialize the logger
    env_logger::init();

    // Parse command line arguments to get the file path
    let file_path = cli::parse_args();

    // Log the file path
    info!("File path: {}", file_path);
}
