use asc100::char::versions::V1_STANDARD;

fn main() {
    // Test specific characters
    let test_chars = vec!['H', 'e', 'l', 'o', ' '];
    
    for ch in test_chars {
        let ascii_val = ch as u32;
        let lookup_idx = V1_STANDARD.lookup[ascii_val as usize];
        println!("Char '{}' (ASCII {}) -> lookup index: {}", ch, ascii_val, lookup_idx);
        
        if lookup_idx != 255 {
            let charset_char = V1_STANDARD.charset[lookup_idx as usize];
            println!("  charset[{}] = '{}'", lookup_idx, charset_char);
        }
    }
    
    // Test simple string
    println!("\nTesting 'Hello':");
    match V1_STANDARD.encode("Hello") {
        Ok(encoded) => {
            println!("Encoded: {}", encoded);
            match V1_STANDARD.decode(&encoded) {
                Ok(decoded) => println!("Decoded: {}", decoded),
                Err(e) => println!("Decode error: {}", e),
            }
        }
        Err(e) => println!("Encode error: {}", e),
    }
}
