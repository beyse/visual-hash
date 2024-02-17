// src/image_generation.rs

use image::{ImageBuffer, Rgba};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use std::str::FromStr;

// Define your color palette
const COLOR_PALETTE: [&str; 10] = [
    "#DBBC76", "#DB9E76", "#7B909C", "#76B8DB", "#9C887B", "#46545C", "#9C917B", "#5C4615",
    "#5C4E46", "#102733",
];

/// Converts a hex color string to an Rgba<u8> value.
fn hex_to_rgba(hex: &str) -> Rgba<u8> {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
    Rgba([r, g, b, 255]) // Alpha is set to 255 (fully opaque)
}

/// Generates a 320x320 pixel image with 16x16 squares, each a different random color from the palette.
/// The random color is picked based on a deterministic RNG seeded with the hash.
pub fn generate_image_with_seed(hash: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let imgx: usize = 320;
    let imgy: usize = 320;
    let square_size: usize = 20; // 320 / 16 = 20

    let mut imgbuf = ImageBuffer::new(imgx as u32, imgy as u32);

    // Convert the hash to a seed for the RNG
    let seed = u64::from_str_radix(&hash[..16], 16).expect("Failed to convert hash to seed");
    let mut rng = Pcg64::seed_from_u64(seed);

    for x in (0..imgx).step_by(square_size) {
        for y in (0..imgy).step_by(square_size) {
            let color_hex = COLOR_PALETTE[rng.gen_range(0..COLOR_PALETTE.len())];
            let color = hex_to_rgba(color_hex);
            for dx in 0..square_size {
                for dy in 0..square_size {
                    imgbuf.put_pixel((x + dx) as u32, (y + dy) as u32, color);
                }
            }
        }
    }

    imgbuf
}
