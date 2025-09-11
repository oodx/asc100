use asc100::char::extensions::{CoreStrategy, ExtensionsStrategy};
use asc100::char::versions::V1_STANDARD;
use asc100::{encode_with_strategy, decode_with_strategy};

#[test]
fn test_core_strategy_treats_markers_as_text() {
    let core_strategy = CoreStrategy::strict();
    
    // Core strategy should treat markers as literal text, not special markers
    let input_with_markers = "Hello #V#name#V# world #EOF#";
    
    println!("Testing Core strategy with markers as literal text...");
    
    let encoded = encode_with_strategy(&input_with_markers, &V1_STANDARD.charset, &V1_STANDARD.lookup, &core_strategy)
        .expect("Core strategy should encode marker text successfully");
    
    let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &core_strategy)
        .expect("Core strategy should decode marker text successfully");
    
    // Should get back exactly the same text (markers not processed)
    assert_eq!(input_with_markers, decoded, "Core strategy should preserve marker text literally");
    println!("✓ Core strategy preserves markers as literal text");
}

#[test]
fn test_strategy_behavior_differences() {
    let core_strategy = CoreStrategy::strict();
    let ext_strategy = ExtensionsStrategy::strict();
    
    let input = "Process #V#data#V# and signal #EOF#";
    
    println!("Comparing Core vs Extensions strategy behavior...");
    
    // Core strategy - treats markers as text
    let core_encoded = encode_with_strategy(&input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &core_strategy)
        .expect("Core encode should succeed");
    let core_decoded = decode_with_strategy(&core_encoded, &V1_STANDARD.charset, &core_strategy)
        .expect("Core decode should succeed");
    
    // Extensions strategy - processes markers
    let ext_encoded = encode_with_strategy(&input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &ext_strategy)
        .expect("Extensions encode should succeed");
    let ext_decoded = decode_with_strategy(&ext_encoded, &V1_STANDARD.charset, &ext_strategy)
        .expect("Extensions decode should succeed");
    
    // Both should decode back to original input
    assert_eq!(input, core_decoded, "Core strategy preserves exact input");
    assert_eq!(input, ext_decoded, "Extensions strategy reconstructs original input");
    
    // But encodings should be different (Core treats #V# as literal chars, Extensions as marker)
    assert_ne!(core_encoded, ext_encoded, "Different strategies should produce different encodings");
    
    println!("✓ Core preserves: {}", core_decoded);
    println!("✓ Extensions reconstructs: {}", ext_decoded);
    println!("✓ Encodings differ as expected");
}

#[test]
fn test_invalid_markers_with_core_strategy() {
    let core_strategy = CoreStrategy::strict();
    
    // Test with text that looks like markers but should be treated as regular text
    let test_cases = vec![
        "#INVALID# not a real marker",
        "#V# incomplete marker without closing",
        "##V## double hashes",
        "#v# lowercase should not match",
        "# V # spaces in marker",
    ];
    
    for input in test_cases {
        println!("Testing invalid marker text: {}", input);
        
        let encoded = encode_with_strategy(input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &core_strategy)
            .expect(&format!("Should encode invalid marker text: {}", input));
        
        let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &core_strategy)
            .expect(&format!("Should decode invalid marker text: {}", input));
        
        assert_eq!(input, decoded, "Invalid marker text should roundtrip exactly");
        println!("  ✓ Preserved as literal text");
    }
}

#[test]
fn test_extensions_strategy_ignores_invalid_markers() {
    let ext_strategy = ExtensionsStrategy::strict();
    
    // Extensions strategy should ignore invalid markers and treat them as text
    let test_cases = vec![
        "#INVALID# not a real marker",
        "#V incomplete marker",
        "##V## double hashes", 
        "#v# wrong case",
    ];
    
    for input in test_cases {
        println!("Testing Extensions strategy with invalid markers: {}", input);
        
        let encoded = encode_with_strategy(input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &ext_strategy)
            .expect(&format!("Should encode invalid marker text: {}", input));
        
        let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &ext_strategy)
            .expect(&format!("Should decode invalid marker text: {}", input));
        
        assert_eq!(input, decoded, "Invalid markers should be treated as literal text");
        println!("  ✓ Invalid marker ignored, treated as text");
    }
}