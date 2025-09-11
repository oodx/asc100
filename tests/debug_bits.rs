use asc100::char::BASE64_LOOKUP;

fn debug_decode(encoded: &str) {
    println!("=== DECODING: {} ===", encoded);
    
    // Step 1: Convert base64 to binary (exact copy from decode function)
    let mut bits = Vec::with_capacity(encoded.len() * 6);
    
    for ch in encoded.chars() {
        let ascii = ch as u32;
        println!("Base64 char: '{}' (ASCII {})", ch, ascii);
        
        if ascii >= 128 {
            println!("  ERROR: Non-ASCII base64 char");
            return;
        }
        
        let value = BASE64_LOOKUP[ascii as usize];
        if value == 255 {
            println!("  ERROR: Invalid base64 char");
            return;
        }
        
        println!("  Base64 value: {}", value);
        
        // Extract 6 bits
        let mut char_bits = Vec::new();
        for i in (0..6).rev() {
            let bit = (value >> i) & 1;
            char_bits.push(bit);
            bits.push(bit);
        }
        println!("  6-bit binary: {:?}", char_bits);
    }
    
    println!("\nTotal bits: {} ({})", bits.len(), bits.iter().map(|b| b.to_string()).collect::<String>());
    
    // Step 2: Extract 7-bit indices
    println!("\n=== EXTRACTING 7-BIT INDICES ===");
    let mut indices = Vec::new();
    for (chunk_idx, chunk) in bits.chunks(7).enumerate() {
        println!("Chunk {}: {:?} (len={})", chunk_idx, chunk, chunk.len());
        
        if chunk.len() == 7 {
            let mut index = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                index |= bit << (6 - i);
            }
            
            println!("  Extracted index: {}", index);
            indices.push(index);
        } else {
            println!("  Skipping incomplete chunk");
        }
    }
    
    println!("\nFinal indices: {:?}", indices);
}

fn main() {
    debug_decode("UZdmze");
}
