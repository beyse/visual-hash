// src/hasher.rs

use log::error;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read};

/// Calculates the SHA-256 hash of the file at the given path.
/// Returns the hash as a hexadecimal string.
pub fn calculate_sha256(file_path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open file: {}", e);
            return Err(e);
        }
    };

    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => {
            hasher.update(buffer);
            let hash_result = hasher.finalize();
            Ok(format!("{:x}", hash_result))
        }
        Err(e) => {
            error!("Failed to read file: {}", e);
            Err(e)
        }
    }
}
