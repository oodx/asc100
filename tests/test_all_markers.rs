use asc100::char::extensions::ExtensionsStrategy;
use asc100::char::versions::V1_STANDARD;
use asc100::{encode_with_strategy, decode_with_strategy};

#[test]
fn test_all_individual_markers() {
    let strategy = ExtensionsStrategy::strict();
    
    // All 19 V1 markers with their expected purposes
    let test_markers = vec![
        // Priority markers (100-106)
        ("#INV#", "Invalid character placeholder"),
        ("#EOF#", "End of file"),
        ("#NL#", "Newline hint"),
        ("#V#", "Variable placeholder"),
        ("#Q#", "Double quote"),
        ("#E#", "Escape/single quote"),
        ("#X#", "Control/validation marker"),
        
        // Stream markers (107-108)
        ("#SSX#", "Start stream"),
        ("#ESX#", "End stream"),
        
        // Content markers (109-115)
        ("#MEM#", "Encoding/transmission metadata"),
        ("#CTX#", "Content/payload context"),
        ("#FX#", "Function/code block"),
        ("#ARG#", "Arguments/parameters"),
        ("#TR#", "Trusted content"),
        ("#DNT#", "Do not trust"),
        ("#BRK#", "Break/separator"),
        
        // Protocol markers (116-118)
        ("#HSO#", "Handshake out"),
        ("#HSI#", "Handshake in"),
        ("#ACK#", "Acknowledge"),
    ];
    
    for (marker, description) in test_markers {
        let input = format!("Start {}content{} End", marker, marker);
        println!("Testing {}: {}", marker, description);
        
        match encode_with_strategy(&input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy) {
            Ok(encoded) => {
                match decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy) {
                    Ok(decoded) => {
                        assert_eq!(input, decoded, "Roundtrip failed for marker {}: {} != {}", marker, input, decoded);
                        println!("  ✓ {} roundtrip successful", marker);
                    }
                    Err(e) => panic!("Decode failed for {}: {}", marker, e),
                }
            }
            Err(e) => panic!("Encode failed for {}: {}", marker, e),
        }
    }
}

#[test]
fn test_multiple_markers_combined() {
    let strategy = ExtensionsStrategy::strict();
    
    // Test combinations of markers
    let test_cases = vec![
        "Hello #V#name#V# world #EOF#",
        "#SSX# Start #V#content#V# #ESX#",
        "#TR# Safe text #TR# #DNT# Dangerous code #DNT#",
        "#FX# function #ARG# param=value #ARG# #FX#",
        "#HSO# handshake #HSI# response #ACK#",
        "#MEM# metadata #CTX# payload #BRK# separator",
        "Quote: #Q#Hello World#Q# Escape: #E#char#E#",
        "#X# Version control #NL# New line hint #INV#",
    ];
    
    for input in test_cases {
        println!("Testing combined: {}", input);
        
        match encode_with_strategy(input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy) {
            Ok(encoded) => {
                match decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy) {
                    Ok(decoded) => {
                        assert_eq!(input, decoded, "Combined markers failed: {} != {}", input, decoded);
                        println!("  ✓ Combined test successful");
                    }
                    Err(e) => panic!("Decode failed for combined test '{}': {}", input, e),
                }
            }
            Err(e) => panic!("Encode failed for combined test '{}': {}", input, e),
        }
    }
}

#[test]
fn test_marker_with_regular_text() {
    let strategy = ExtensionsStrategy::strict();
    
    // Test markers mixed with various text content
    let test_cases = vec![
        "fn main() { #FX# println!(\"Hello\"); #FX# }",
        "user: john@example.com #V#email#V# pass: secret123",
        "Config: debug=true #MEM# #CTX# app_data #CTX#",
        "Numbers 123 and symbols !@# with #TR# trusted #TR#",
        "Whitespace\t\nHandling #NL# With #EOF# markers",
    ];
    
    for input in test_cases {
        println!("Testing mixed content: {}", input);
        
        let encoded = encode_with_strategy(input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy)
            .expect(&format!("Failed to encode: {}", input));
        
        let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy)
            .expect(&format!("Failed to decode: {}", input));
        
        assert_eq!(input, decoded, "Mixed content failed: {} != {}", input, decoded);
        println!("  ✓ Mixed content successful");
    }
}

#[test]
fn test_nested_and_complex_markers() {
    let strategy = ExtensionsStrategy::strict();
    
    // Test complex scenarios
    let complex_input = "#SSX# #MEM# version=1.0 #MEM# #CTX# #TR# User: #V#username#V# #TR# #FX# process(#ARG# input=#V#data#V# #ARG#) #FX# #CTX# #ESX#";
    
    println!("Testing complex nested markers...");
    
    let encoded = encode_with_strategy(&complex_input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy)
        .expect("Failed to encode complex input");
    
    let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy)
        .expect("Failed to decode complex input");
    
    assert_eq!(complex_input, decoded, "Complex nested failed: {} != {}", complex_input, decoded);
    println!("✓ Complex nested markers successful");
}