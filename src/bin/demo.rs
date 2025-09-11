use std::time::Instant;
use asc100::char::versions::{V1_STANDARD, V2_NUMBERS, V3_LOWERCASE, V4_URL};
use asc100::char::extensions::{CoreStrategy, ExtensionsStrategy};
use asc100::{encode_with_strategy, decode_with_strategy};

fn main() {
    println!("🦅 ASC100 v0.2.0 - Executive Strategy Demonstration");
    println!("═══════════════════════════════════════════════════");
    println!();

    // Test cases showing different scenarios
    let test_cases = vec![
        ("Basic ASCII", "Hello, World!"),
        ("With Numbers", "Data: 12345, Value: 67890"),
        ("Mixed Content", "Source code: fn test() { return 42; }"),
        ("URL Content", "https://example.com/path?query=value&foo=bar"),
        ("With Markers", "Start #SSX# content #EOF# end"),
        ("Invalid Unicode", "Hello🌍World"), // Contains non-ASCII
        ("Whitespace Heavy", "Text\t\nwith\r\nvarious\twhitespace"),
    ];

    for (name, input) in test_cases {
        println!("📋 Test Case: {}", name);
        println!("Input: \"{}\"", input);
        println!();

        // Show strategy comparison
        demonstrate_strategies(input);
        
        // Show charset performance comparison
        if input.chars().all(|c| c.is_ascii()) {
            demonstrate_charsets(input);
        }
        
        println!("{}", "─".repeat(60));
        println!();
    }

    println!("🎯 Summary: ASC100 provides 87.5% bit efficiency with flexible");
    println!("   invalid character handling through strategy patterns.");
}

fn demonstrate_strategies(input: &str) {
    println!("🔧 Strategy Comparison:");
    
    // Strict Strategy - Error on invalid
    let strict_strategy = CoreStrategy::strict();
    match encode_with_strategy(input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strict_strategy) {
        Ok(encoded) => {
            let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &strict_strategy).unwrap();
            println!("  STRICT:   ✅ \"{}\" ({}% compression)", decoded, compression_ratio(input, &encoded));
        }
        Err(e) => {
            println!("  STRICT:   ❌ Error - {}", e);
        }
    }

    // Strip Strategy - Remove invalid
    let strip_strategy = CoreStrategy::strip();
    match encode_with_strategy(input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strip_strategy) {
        Ok(encoded) => {
            let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &strip_strategy).unwrap();
            println!("  STRIP:    ✅ \"{}\" ({}% compression)", decoded, compression_ratio(input, &encoded));
        }
        Err(e) => {
            println!("  STRIP:    ❌ Error - {}", e);
        }
    }

    // Sanitize Strategy - Replace with #INV#
    let sanitize_strategy = ExtensionsStrategy::sanitize();
    match encode_with_strategy(input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &sanitize_strategy) {
        Ok(encoded) => {
            let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &sanitize_strategy).unwrap();
            println!("  SANITIZE: ✅ \"{}\" ({}% compression)", decoded, compression_ratio(input, &encoded));
        }
        Err(e) => {
            println!("  SANITIZE: ❌ Error - {}", e);
        }
    }
    
    println!();
}

fn demonstrate_charsets(input: &str) {
    println!("📊 Character Set Performance:");
    
    let charsets = vec![
        ("V1_STANDARD", &V1_STANDARD),
        ("V2_NUMBERS", &V2_NUMBERS),
        ("V3_LOWERCASE", &V3_LOWERCASE),
        ("V4_URL", &V4_URL),
    ];

    let strategy = CoreStrategy::strict();
    
    for (name, charset) in charsets {
        let start = Instant::now();
        match encode_with_strategy(input, &charset.charset, &charset.lookup, &strategy) {
            Ok(encoded) => {
                let duration = start.elapsed();
                let ratio = compression_ratio(input, &encoded);
                println!("  {}: {}% compression, {:.2}ms", name, ratio, duration.as_secs_f64() * 1000.0);
            }
            Err(_) => {
                println!("  {}: Failed (contains unsupported characters)", name);
            }
        }
    }
    
    println!();
}

fn compression_ratio(input: &str, encoded: &str) -> u32 {
    let input_size = input.len();
    let encoded_size = encoded.len();
    if input_size == 0 { return 100; }
    ((encoded_size as f64 / input_size as f64) * 100.0).round() as u32
}