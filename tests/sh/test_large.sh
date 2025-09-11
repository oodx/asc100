#!/bin/bash

# Test with a large code file
echo "Testing ASC100 with large code blob..."
echo ""

# Create a test file with actual code
cat > /tmp/test_code.txt << 'EOF'
// Complex Rust code example
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct DataProcessor {
    cache: HashMap<String, Vec<u8>>,
    config: ProcessorConfig,
}

impl DataProcessor {
    pub fn new(config: ProcessorConfig) -> Self {
        Self {
            cache: HashMap::new(),
            config,
        }
    }
    
    pub fn process(&mut self, input: &str) -> Result<String, Box<dyn Error>> {
        // Check cache first
        if let Some(cached) = self.cache.get(input) {
            return Ok(String::from_utf8_lossy(cached).to_string());
        }
        
        // Process the input
        let mut result = String::new();
        for (i, ch) in input.chars().enumerate() {
            match ch {
                'a'..='z' => result.push(ch.to_ascii_uppercase()),
                'A'..='Z' => result.push(ch.to_ascii_lowercase()),
                '0'..='9' => {
                    let digit = ch.to_digit(10).unwrap();
                    result.push_str(&format!("[{}]", digit));
                }
                ' ' | '\t' | '\n' => result.push(ch),
                _ => result.push_str(&format!("\\x{:02x}", ch as u8)),
            }
        }
        
        // Cache the result
        self.cache.insert(input.to_string(), result.as_bytes().to_vec());
        Ok(result)
    }
}

fn main() {
    println!("Hello, World!");
    let data = vec![1, 2, 3, 4, 5];
    let sum: i32 = data.iter().sum();
    println!("Sum: {}", sum);
}
EOF

# Create a simple Rust program to test encoding
cat > /tmp/test_encoder.rs << 'EOF'
use asc100::versions::V1_STANDARD;
use std::fs;

fn main() {
    let input = fs::read_to_string("/tmp/test_code.txt").unwrap();
    
    println!("Original size: {} bytes", input.len());
    
    match V1_STANDARD.encode(&input) {
        Ok(encoded) => {
            println!("Encoded size:  {} bytes", encoded.len());
            println!("Ratio: {:.2}x", encoded.len() as f64 / input.len() as f64);
            println!("");
            
            // Test decode
            match V1_STANDARD.decode(&encoded) {
                Ok(decoded) => {
                    if decoded == input {
                        println!("✓ Roundtrip successful!");
                        println!("First 100 chars of encoded: {}", &encoded[..100.min(encoded.len())]);
                    } else {
                        println!("✗ Roundtrip failed!");
                        println!("Mismatch at character {}", 
                            input.chars().zip(decoded.chars())
                                .position(|(a, b)| a != b)
                                .unwrap_or(0));
                    }
                }
                Err(e) => println!("✗ Decode error: {}", e),
            }
        }
        Err(e) => println!("✗ Encode error: {}", e),
    }
}
EOF

# Compile and run
echo "Compiling test..."
rustc --edition 2021 -L target/debug/deps /tmp/test_encoder.rs -o /tmp/test_encoder --extern asc100=target/debug/libasc100.rlib

echo "Running test..."
echo "================================"
/tmp/test_encoder
echo "================================"

# Cleanup
rm -f /tmp/test_code.txt /tmp/test_encoder.rs /tmp/test_encoder
