use asc100::char::versions::V1_STANDARD;

fn main() {
    // Debug the binary encoding for 'Hello'
    let input = "Hello";
    
    // Manually trace through encode process
    println!("=== ENCODING ===");
    for (i, ch) in input.chars().enumerate() {
        let ascii = ch as u32;
        let lookup_idx = V1_STANDARD.lookup[ascii as usize];
        println!("Char[{}] '{}' -> index {}", i, ch, lookup_idx);
        
        // Show 7-bit binary
        let mut bits = Vec::new();
        for bit_pos in (0..7).rev() {
            bits.push((lookup_idx >> bit_pos) & 1);
        }
        println!("  Binary: {:?} ({})", bits, bits.iter().map(|b| b.to_string()).collect::<String>());
    }
    
    // Now try to decode manually
    println!("\n=== MANUAL DECODE TEST ===");
    let encoded = V1_STANDARD.encode(input).unwrap();
    println!("Encoded: {}", encoded);
    
    // Manual decode to debug
    println!("Decoding {} characters...", encoded.chars().count());
    
    let decoded = V1_STANDARD.decode(&encoded).unwrap();
    println!("Final decoded: '{}'", decoded);
}
