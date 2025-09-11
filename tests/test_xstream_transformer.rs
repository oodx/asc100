#[cfg(feature = "xstream")]
mod transformer_tests {
    use asc100::xstream_transformer::{transformers, pipeline, integration, TransformMode, Asc100Transformer};

    #[test]
    fn test_encoder_transformer() {
        let transformer = transformers::encoder_key();
        let input = "user=john@example.com; pass=secret123; mode=debug";
        
        let result = pipeline::transform_stream(input, &transformer)
            .expect("Should transform stream");
        
        println!("Input:  {}", input);
        println!("Output: {}", result);
        
        // Should have :asc suffixes on all keys
        assert!(result.contains("user:asc="));
        assert!(result.contains("pass:asc="));
        assert!(result.contains("mode:asc="));
        
        // Values should be Base64 encoded
        assert!(!result.contains("john@example.com"));
        assert!(!result.contains("secret123"));
        assert!(!result.contains("debug"));
    }

    #[test]
    fn test_decoder_transformer() {
        let encoder = transformers::encoder_key();
        let decoder = transformers::decoder();
        
        let original = "content=Hello, World!; app:version=1.0";
        
        // Encode first
        let encoded = pipeline::transform_stream(original, &encoder)
            .expect("Should encode");
        
        println!("Original: {}", original);
        println!("Encoded:  {}", encoded);
        
        // Then decode
        let decoded = pipeline::transform_stream(&encoded, &decoder)
            .expect("Should decode");
        
        println!("Decoded:  {}", decoded);
        
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_bidirectional_transformer() {
        let encoder = transformers::encoder_key();
        let bidirectional = transformers::bidirectional();
        
        // First create a properly encoded token
        let test_encoded = pipeline::transform_stream("encoded=Test", &encoder)
            .expect("Should create test token");
        
        // Extract just the encoded part
        let encoded_part = test_encoded.split('=').nth(1).unwrap();
        let input = format!("normal=text; encoded:asc={}; mixed=value", encoded_part);
        
        let result = pipeline::transform_stream(&input, &bidirectional)
            .expect("Should handle bidirectional");
        
        println!("Input:  {}", input);
        println!("Output: {}", result);
        
        // Should decode the encoded token and encode the others
        assert!(result.contains("normal:asc="));  // Should be encoded
        assert!(result.contains("encoded=Test"));  // Should be decoded
        assert!(result.contains("mixed:asc="));    // Should be encoded
    }

    #[test]
    fn test_extensions_with_templates() {
        let transformer = transformers::extensions_encoder();
        let template_content = "Hello #V#name#V#, your order #EOF#";
        let input = format!("template={}", template_content);
        
        let result = pipeline::transform_stream(&input, &transformer)
            .expect("Should encode template");
        
        println!("Template: {}", input);
        println!("Encoded:  {}", result);
        
        assert!(result.contains("template:asc="));
        
        // Decode to verify roundtrip
        let decoder = transformers::extensions_decoder();
        let decoded = pipeline::transform_stream(&result, &decoder)
            .expect("Should decode template");
        
        println!("Decoded:  {}", decoded);
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_selective_transformation() {
        let transformer = transformers::encoder_key();
        let input = "user=john; pass=secret; debug=true; temp=data";
        
        // Only encode specific keys
        let result = pipeline::transform_selective(input, &transformer, &["user", "pass"])
            .expect("Should selectively transform");
        
        println!("Input:      {}", input);
        println!("Selective:  {}", result);
        
        // Should only encode user and pass
        assert!(result.contains("user:asc="));
        assert!(result.contains("pass:asc="));
        assert!(result.contains("debug=true"));  // unchanged
        assert!(result.contains("temp=data"));   // unchanged
    }

    #[test]
    fn test_compression_gate() {
        let transformer = transformers::encoder_key();
        
        // Small content - should not be compressed
        let small = "key=val";
        let small_result = integration::compression_gate(small, &transformer, 20)
            .expect("Should handle small content");
        assert_eq!(small, small_result);  // unchanged
        
        // Large content - should be compressed
        let large = format!("content={}", "This is a large piece of content. ".repeat(10));
        let large_result = integration::compression_gate(&large, &transformer, 20)
            .expect("Should handle large content");
        assert_ne!(large, large_result);  // should be encoded
        assert!(large_result.contains("content:asc="));
    }

    #[test]
    fn test_fork_processing() {
        let transformer = transformers::encoder_value();  // Use value suffix mode
        let input = "data=important information; config=settings";
        
        let (original, encoded) = integration::fork_encode(input, &transformer)
            .expect("Should fork processing");
        
        println!("Original: {}", original);
        println!("Encoded:  {}", encoded);
        
        assert_eq!(input, original);
        assert!(encoded.contains(":a"));  // value suffix mode
        assert_ne!(original, encoded);
    }

    #[test]
    fn test_chained_operations() {
        let encoder = transformers::encoder_key();
        let input = "message=Hello World; user=alice";
        
        // Chain ASC100 encoding with a mock operation
        let result = pipeline::chain_transform(input, &encoder, |intermediate| {
            // Mock operation: add a timestamp token
            Ok(format!("{}; timestamp=1234567890", intermediate))
        }).expect("Should chain operations");
        
        println!("Input:   {}", input);
        println!("Chained: {}", result);
        
        assert!(result.contains("message:asc="));
        assert!(result.contains("user:asc="));
        assert!(result.contains("timestamp=1234567890"));
    }

    #[test]
    fn test_large_content_performance() {
        let transformer = transformers::encoder_key();
        
        // Generate large content with markers
        let large_content = format!(
            "#SSX# {} #V#data#V# {} #ESX#",
            "Large content section. ".repeat(100),
            "End section with markers. ".repeat(50)
        );
        let input = format!("payload={}", large_content);
        
        let start = std::time::Instant::now();
        let encoded = pipeline::transform_stream(&input, &transformer)
            .expect("Should encode large content");
        let encode_time = start.elapsed();
        
        let decoder = transformers::decoder();
        let start = std::time::Instant::now();
        let decoded = pipeline::transform_stream(&encoded, &decoder)
            .expect("Should decode large content");
        let decode_time = start.elapsed();
        
        println!("Original size: {} bytes", input.len());
        println!("Encoded size:  {} bytes", encoded.len());
        println!("Encode time:   {:?}", encode_time);
        println!("Decode time:   {:?}", decode_time);
        
        assert_eq!(input, decoded);
        assert!(encode_time.as_millis() < 100, "Encoding should be fast");
        assert!(decode_time.as_millis() < 100, "Decoding should be fast");
    }

    #[test]
    fn test_namespace_handling() {
        let transformer = transformers::encoder_key();
        let input = "app:config=debug; user:name=alice; global=setting";
        
        let result = pipeline::transform_stream(input, &transformer)
            .expect("Should handle namespaces");
        
        println!("Input:  {}", input);
        println!("Output: {}", result);
        
        // Should preserve namespaces in keys
        assert!(result.contains("app:config:asc="));
        assert!(result.contains("user:name:asc="));
        assert!(result.contains("global:asc="));
    }
}