use asc100::char::versions::V1_STANDARD;
use asc100::char::extensions::{CoreStrategy, ExtensionsStrategy};
use asc100::{encode_with_strategy, decode_with_strategy, Asc100Error};

#[test]
fn test_invalid_character_error_strategy() {
    // Test that invalid characters cause errors when using StrictFilter strategy
    let strategy = CoreStrategy::strict();
    
    // Test with high ASCII character that's definitely invalid
    let invalid_input = "Hello\u{0080}World"; // Unicode > 127
    
    match encode_with_strategy(invalid_input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy) {
        Err(Asc100Error::InvalidCharacter(_)) | Err(Asc100Error::InvalidCharacterWithContext { .. }) => {
            // Expected behavior for invalid character (both old and new error types)
        }
        Ok(_) => panic!("Should have failed with invalid character"),
        Err(e) => panic!("Unexpected error type: {}", e),
    }
}

#[test]
fn test_invalid_character_ignore_strategy() {
    // Test that invalid characters are ignored (removed from output) using StripFilter
    let strategy = CoreStrategy::strip();
    
    // Test with Unicode character that will be stripped
    let input = "Hello\u{0080}World";
    let expected_without_invalid = "HelloWorld";
    
    let encoded = encode_with_strategy(&input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy)
        .expect("Should encode with strip strategy");
    let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy)
        .expect("Should decode successfully");
    
    assert_eq!(decoded, expected_without_invalid, "Invalid character should be stripped");
}

#[test]
fn test_invalid_character_replace_with_inv_strategy() {
    // Test that invalid characters are replaced with #INV# marker using SanitizeFilter
    let strategy = ExtensionsStrategy::sanitize();
    
    // Test with Unicode character that will be replaced
    let input = "Hello\u{0080}World";
    
    let encoded = encode_with_strategy(&input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy)
        .expect("Should encode with sanitize strategy");
    let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy)
        .expect("Should decode successfully");
    
    // Should contain #INV# marker where invalid character was
    assert!(decoded.contains("#INV#"), "Should contain #INV# marker, got: {}", decoded);
    assert!(decoded.contains("Hello"), "Should contain valid parts");
    assert!(decoded.contains("World"), "Should contain valid parts");
}

#[test]
fn test_inv_marker_roundtrip() {
    // Test that #INV# markers are properly handled in roundtrip
    let strategy = ExtensionsStrategy::strict();
    
    let input_with_inv = "Hello #INV# World";
    
    let encoded = encode_with_strategy(input_with_inv, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy)
        .expect("Should encode #INV# marker");
    let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy)
        .expect("Should decode #INV# marker");
    
    assert_eq!(input_with_inv, decoded, "#INV# marker should roundtrip correctly");
}

#[test]
fn test_multiple_invalid_characters() {
    // Test handling multiple invalid characters in one string
    let strategy = ExtensionsStrategy::sanitize();
    
    // Use two different Unicode characters
    let input = "Start\u{0080}Middle\u{0081}End";
    
    let encoded = encode_with_strategy(&input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy)
        .expect("Should encode multiple invalid chars");
    let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy)
        .expect("Should decode multiple invalid chars");
    
    // Should have two #INV# markers
    let inv_count = decoded.matches("#INV#").count();
    assert_eq!(inv_count, 2, "Should have exactly 2 #INV# markers, got: {}", decoded);
    assert!(decoded.contains("Start"), "Should contain valid parts");
    assert!(decoded.contains("Middle"), "Should contain valid parts");
    assert!(decoded.contains("End"), "Should contain valid parts");
}

#[test]
fn test_mixed_valid_invalid_markers() {
    // Test string with valid markers, invalid characters, and regular text
    let strategy = ExtensionsStrategy::sanitize();
    
    // Use Unicode character mixed with valid markers
    let input = "Start #V# Middle \u{0080} #EOF# End";
    
    let encoded = encode_with_strategy(&input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy)
        .expect("Should encode mixed content");
    let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy)
        .expect("Should decode mixed content");
    
    // Should preserve valid markers and replace invalid char
    assert!(decoded.contains("#V#"), "Should preserve #V# marker");
    assert!(decoded.contains("#EOF#"), "Should preserve #EOF# marker");
    assert!(decoded.contains("#INV#"), "Should replace invalid char with #INV#");
    assert!(decoded.contains("Start"), "Should contain valid text");
    assert!(decoded.contains("Middle"), "Should contain valid text");
    assert!(decoded.contains("End"), "Should contain valid text");
}

#[test]
fn test_strategy_differences_with_invalid_chars() {
    // Test that different strategies handle the same invalid input differently
    let input = "Test\u{0080}Input";
    
    // Strict strategy should fail
    let error_strategy = CoreStrategy::strict();
    assert!(
        encode_with_strategy(&input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &error_strategy).is_err(),
        "Strict strategy should fail with invalid character"
    );
    
    // Strip strategy should succeed and remove invalid char
    let strip_strategy = CoreStrategy::strip();
    let strip_result = encode_with_strategy(&input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strip_strategy)
        .expect("Strip strategy should succeed");
    let strip_decoded = decode_with_strategy(&strip_result, &V1_STANDARD.charset, &strip_strategy)
        .expect("Should decode stripped result");
    assert_eq!(strip_decoded, "TestInput", "Should strip invalid character");
    
    // Sanitize strategy should succeed and add #INV#
    let sanitize_strategy = ExtensionsStrategy::sanitize();
    let sanitize_result = encode_with_strategy(&input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &sanitize_strategy)
        .expect("Sanitize strategy should succeed");
    let sanitize_decoded = decode_with_strategy(&sanitize_result, &V1_STANDARD.charset, &sanitize_strategy)
        .expect("Should decode sanitized result");
    assert!(sanitize_decoded.contains("#INV#"), "Should replace with #INV# marker");
    assert!(sanitize_decoded.contains("Test"), "Should contain valid parts");
    assert!(sanitize_decoded.contains("Input"), "Should contain valid parts");
}

#[test]
fn test_edge_case_only_invalid_characters() {
    // Test string containing only invalid characters
    let strategy = ExtensionsStrategy::sanitize();
    
    // Use Unicode characters for all-invalid string
    let input = "\u{0080}\u{0081}\u{0082}";
    
    let encoded = encode_with_strategy(&input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy)
        .expect("Should encode all-invalid string");
    let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy)
        .expect("Should decode all-invalid string");
    
    // Should be all #INV# markers
    let inv_count = decoded.matches("#INV#").count();
    assert_eq!(inv_count, 3, "Should have one #INV# per invalid char");
}