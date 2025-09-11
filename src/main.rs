use asc100::char::versions::{V1_STANDARD, V2_NUMBERS, V3_LOWERCASE, V4_URL};

fn test_version(version: &asc100::char::versions::Asc100Version, test_text: &str) {
    println!("\n=== Testing {} ===", version.name);
    
    // Show first part of charset
    version.display_charset();
    
    // Test encoding
    match version.encode(test_text) {
        Ok(encoded) => {
            println!("\nOriginal ({} chars): {}", test_text.len(), test_text);
            println!("Encoded  ({} chars): {}", encoded.len(), encoded);
            
            // Test decoding
            match version.decode(&encoded) {
                Ok(decoded) => {
                    println!("Decoded  ({} chars): {}", decoded.len(), decoded);
                    
                    if decoded == test_text {
                        println!("✓ Roundtrip successful!");
                        println!("  Compression ratio: {:.2}x", 
                                 encoded.len() as f64 / test_text.len() as f64);
                    } else {
                        println!("✗ Roundtrip failed!");
                        println!("  Expected: {:?}", test_text);
                        println!("  Got:      {:?}", decoded);
                    }
                }
                Err(e) => println!("✗ Decode error: {}", e),
            }
        }
        Err(e) => println!("✗ Encode error: {}", e),
    }
}

fn main() {
    println!("ASC100 Encoding System - Roundtrip Test\n");
    println!("========================================");
    
    let test_cases = vec![
        "Hello, World!",
        "The quick brown fox jumps over the lazy dog",
        "1234567890",
        "~!@#$%^&*()_+-=[]{}|;:'\",.<>?/",
        "function test() { return 42; }",
        "https://example.com/path?query=value&foo=bar",
        "\tIndented\n\tText\r\n",
        "~ Space and tilde test ~",
    ];
    
    // Test all versions with a sample text
    let sample = "Hello, World! 123";
    for version in [&V1_STANDARD, &V2_NUMBERS, &V3_LOWERCASE, &V4_URL] {
        test_version(version, sample);
    }
    
    // Full test suite with V1
    println!("\n\n=== Full Test Suite with V1_STANDARD ===");
    println!("==========================================");
    
    let mut passed = 0;
    let mut failed = 0;
    
    for test_text in &test_cases {
        println!("\nTest: {:?}", test_text);
        match V1_STANDARD.encode(test_text) {
            Ok(encoded) => {
                match V1_STANDARD.decode(&encoded) {
                    Ok(decoded) => {
                        if &decoded == test_text {
                            println!("  ✓ PASS (ratio: {:.2}x)", 
                                     encoded.len() as f64 / test_text.len() as f64);
                            passed += 1;
                        } else {
                            println!("  ✗ FAIL: Mismatch");
                            println!("    Expected: {:?}", test_text);
                            println!("    Got:      {:?}", decoded);
                            failed += 1;
                        }
                    }
                    Err(e) => {
                        println!("  ✗ FAIL: Decode error: {}", e);
                        failed += 1;
                    }
                }
            }
            Err(e) => {
                println!("  ✗ FAIL: Encode error: {}", e);
                failed += 1;
            }
        }
    }
    
    println!("\n========================================");
    println!("Results: {} passed, {} failed", passed, failed);
    
    if failed == 0 {
        println!("✓ All tests passed!");
    } else {
        println!("✗ Some tests failed");
        std::process::exit(1);
    }
}