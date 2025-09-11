use asc100::char::versions::V1_STANDARD;
use asc100::char::extensions::{CoreStrategy, ExtensionsStrategy};

#[test]
fn test_convenience_api_encode_with() {
    let input = "Hello, World!";
    
    // Test CoreStrategy::strip convenience
    let strip_strategy = CoreStrategy::strip();
    let result = V1_STANDARD.encode_with(input, &strip_strategy)
        .expect("Should encode with strip strategy");
    
    let decoded = V1_STANDARD.decode_with(&result, &strip_strategy)
        .expect("Should decode with strip strategy");
    
    assert_eq!(input, decoded);
}

#[test]
fn test_convenience_api_extensions_strategy() {
    let input = "Start #SSX# content #EOF# end";
    
    // Test ExtensionsStrategy::strict convenience
    let strategy = ExtensionsStrategy::strict();
    let result = V1_STANDARD.encode_with(input, &strategy)
        .expect("Should encode with extensions strategy");
    
    let decoded = V1_STANDARD.decode_with(&result, &strategy)
        .expect("Should decode with extensions strategy");
    
    assert_eq!(input, decoded);
}

#[test]
fn test_convenience_api_invalid_character_handling() {
    let input = "Hello\u{0080}World"; // Contains invalid Unicode
    
    // Test strip strategy via convenience API
    let strip_strategy = CoreStrategy::strip();
    let result = V1_STANDARD.encode_with(input, &strip_strategy)
        .expect("Should encode with strip strategy");
    
    let decoded = V1_STANDARD.decode_with(&result, &strip_strategy)
        .expect("Should decode with strip strategy");
    
    assert_eq!("HelloWorld", decoded); // Invalid character stripped
}

#[test]
fn test_convenience_api_sanitize_strategy() {
    let input = "Hello\u{0080}World"; // Contains invalid Unicode
    
    // Test sanitize strategy via convenience API
    let sanitize_strategy = ExtensionsStrategy::sanitize();
    let result = V1_STANDARD.encode_with(input, &sanitize_strategy)
        .expect("Should encode with sanitize strategy");
    
    let decoded = V1_STANDARD.decode_with(&result, &sanitize_strategy)
        .expect("Should decode with sanitize strategy");
    
    assert!(decoded.contains("#INV#")); // Invalid character replaced with marker
    assert!(decoded.contains("Hello"));
    assert!(decoded.contains("World"));
}

#[test]
fn test_api_comparison_old_vs_new() {
    let input = "Test content";
    let strategy = CoreStrategy::strict();
    
    // Old API (verbose)
    let old_result = asc100::encode_with_strategy(
        input, 
        &V1_STANDARD.charset, 
        &V1_STANDARD.lookup, 
        &strategy
    ).expect("Should encode with old API");
    
    // New convenience API
    let new_result = V1_STANDARD.encode_with(input, &strategy)
        .expect("Should encode with new API");
    
    // Should produce identical results
    assert_eq!(old_result, new_result);
}