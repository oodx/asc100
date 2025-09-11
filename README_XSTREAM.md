# ASC100 XStream Integration Guide

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Complete guide for integrating ASC100 encoding with XStream token pipelines. This integration provides efficient, URL-safe encoding for XStream tokens while maintaining full compatibility with existing XStream operations like merge, fork, gate, and transform.

## Table of Contents

- [Quick Start](#quick-start)
- [Integration Approaches](#integration-approaches)
- [XStream Pipeline Patterns](#xstream-pipeline-patterns)
- [Encoding Strategies](#encoding-strategies)
- [Template System Usage](#template-system-usage)
- [Performance Considerations](#performance-considerations)
- [Real-World Examples](#real-world-examples)
- [API Reference](#api-reference)

## Quick Start

Add ASC100 with XStream support to your `Cargo.toml`:

```toml
[dependencies]
asc100 = { version = "0.1.0", features = ["xstream"] }
```

### Basic Token Encoding

```rust
use asc100::xstream_simple::{presets, utils};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let encoder = presets::core_key();
    let tokens = "user=john@example.com; pass=secret123; config=debug";
    
    // Encode all token values
    let encoded = utils::encode_token_string(tokens, &encoder)?;
    println!("Encoded: {}", encoded);
    // Output: "user_asc=am9obkBleGFtcGxlLmNvbQ==; pass_asc=c2VjcmV0MTIz; config_asc=ZGVidWc="
    
    // Decode back to original
    let decoded = utils::decode_token_string(&encoded, &encoder)?;
    println!("Decoded: {}", decoded);
    // Output: "user=john@example.com; pass=secret123; config=debug"
    
    Ok(())
}
```

### Transformer Pipeline Integration

```rust
use asc100::xstream_transformer::{transformers, pipeline};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let encoder = transformers::encoder_key();
    let decoder = transformers::decoder();
    
    let original = "message=Hello, World!; user=alice; token=xyz123";
    
    // Encode through transformer
    let encoded = pipeline::transform_stream(original, &encoder)?;
    println!("Encoded: {}", encoded);
    
    // Decode through transformer
    let decoded = pipeline::transform_stream(&encoded, &decoder)?;
    println!("Decoded: {}", decoded);
    
    assert_eq!(original, decoded);
    Ok(())
}
```

## Integration Approaches

ASC100 provides two complementary approaches for XStream integration:

### 1. Simple Integration (`xstream_simple`)

**Best for:** Existing applications that need to add ASC100 encoding with minimal changes.

```rust
use asc100::xstream_simple::{Asc100ValueEncoder, Asc100Mode};

let encoder = Asc100ValueEncoder::core(Asc100Mode::KeySuffix);

// Encode individual key-value pairs
let (key, value) = encoder.encode_kv_pair("password", "secret123")?;
// Result: ("password_asc", "c2VjcmV0MTIz")

// Auto-detect and decode
let (clean_key, decoded) = encoder.decode_kv_pair("password_asc", "c2VjcmV0MTIz")?;
// Result: ("password", "secret123")
```

### 2. Transformer Integration (`xstream_transformer`)

**Best for:** Applications using XStream pipelines that need seamless transformer composition.

```rust
use asc100::xstream_transformer::{Asc100Transformer, TransformMode};

let transformer = Asc100Transformer::core(TransformMode::Bidirectional);

// Transform based on current state
let (key, value) = transformer.transform_value("data", "content")?;
// If unmarked: ("data_asc", "Y29udGVudA==")
// If marked: auto-decodes back to original
```

## XStream Pipeline Patterns

### Basic Pipeline Operations

#### Chain Transformations
Compose ASC100 with other XStream operations:

```rust
use asc100::xstream_transformer::{transformers, pipeline};

let encoder = transformers::encoder_key();
let input = "user=john; data=sensitive";

// Chain with custom operations
let result = pipeline::chain_transform(input, &encoder, |intermediate| {
    // Add timestamp after encoding
    Ok(format!("{}; timestamp={}", intermediate, 1234567890))
})?;

println!("{}", result);
// Output: "user_asc=am9obg==; data_asc=c2Vuc2l0aXZl; timestamp=1234567890"
```

#### Selective Transformation
Apply ASC100 only to specific keys:

```rust
let transformer = transformers::encoder_key();
let input = "user=john; pass=secret; debug=true; temp=data";

// Only encode user and pass fields
let result = pipeline::transform_selective(
    input, 
    &transformer, 
    &["user", "pass"]
)?;

println!("{}", result);
// Output: "user_asc=am9obg==; pass_asc=c2VjcmV0; debug=true; temp=data"
```

### Fork and Merge Operations

#### Fork Processing
Create encoded and plain branches:

```rust
use asc100::xstream_transformer::integration;

let transformer = transformers::encoder_value();
let input = "data=important; config=settings";

let (original, encoded) = integration::fork_encode(input, &transformer)?;

println!("Original: {}", original);
// Output: "data=important; config=settings"

println!("Encoded: {}", encoded);  
// Output: "data=aW1wb3J0YW50:a; config=c2V0dGluZ3M=:a"
```

#### Merge Streams
Combine encoded and decoded streams:

```rust
let merged = integration::merge_streams(
    &encoded_stream,
    &decoded_stream,
    true  // prefer_encoded
)?;
```

### Gate Operations

#### Compression Gate
Only encode large content:

```rust
let transformer = transformers::encoder_key();

// Small content - passes through unchanged
let small = "key=val";
let result = integration::compression_gate(small, &transformer, 20)?;
assert_eq!(small, result);

// Large content - gets encoded
let large = format!("content={}", "Large data ".repeat(10));
let result = integration::compression_gate(&large, &transformer, 20)?;
assert!(result.contains("content_asc="));
```

## Encoding Strategies

### Core vs Extensions Strategies

#### Core Strategy - Basic Encoding
Best for simple data without special markers:

```rust
use asc100::xstream_simple::presets;

let encoder = presets::core_key();
let tokens = "user=alice; data=simple text";
let encoded = utils::encode_token_string(tokens, &encoder)?;
```

#### Extensions Strategy - Template Support
Supports template markers like `#V#`, `#EOF#`:

```rust
let encoder = presets::extensions_key();
let template = "msg=Hello #V#name#V#, order #EOF#";
let encoded = utils::encode_token_string(template, &encoder)?;

// Template markers are preserved and properly encoded
```

### Encoding Modes

#### Key Suffix Mode (Recommended)
Adds `_asc` to keys to indicate encoding:

```rust
use asc100::xstream_simple::{Asc100ValueEncoder, Asc100Mode};

let encoder = Asc100ValueEncoder::core(Asc100Mode::KeySuffix);
let (key, value) = encoder.encode_kv_pair("password", "secret")?;
// Result: ("password_asc", "c2VjcmV0")
```

#### Value Suffix Mode
Adds `:a` to values to indicate encoding:

```rust
let encoder = Asc100ValueEncoder::core(Asc100Mode::ValueSuffix);
let (key, value) = encoder.encode_kv_pair("password", "secret")?;
// Result: ("password", "c2VjcmV0:a")
```

#### Bidirectional Mode
Maximum compatibility - uses both indicators:

```rust
let encoder = Asc100ValueEncoder::core(Asc100Mode::Both);
let (key, value) = encoder.encode_kv_pair("password", "secret")?;
// Result: ("password_asc", "c2VjcmV0:a")
```

## Template System Usage

ASC100's Extensions strategy supports XStream template markers:

### Supported Template Markers

| Marker | Index | Purpose | Example |
|--------|-------|---------|---------|
| `#V#` | Variable placeholders | `Hello #V#name#V#` |
| `#EOF#` | 102 | End marker | `Data content #EOF#` |
| `#SSX#` | 100 | Start stream | `#SSX# stream data` |
| `#ESX#` | 101 | End stream | `stream data #ESX#` |

### Template Encoding Example

```rust
use asc100::xstream_transformer::transformers;

let transformer = transformers::extensions_encoder();
let template = "Hello #V#name#V#, your order #EOF#";
let input = format!("template={}", template);

let encoded = pipeline::transform_stream(&input, &transformer)?;
println!("Encoded: {}", encoded);

// Decode preserves template structure
let decoder = transformers::extensions_decoder();
let decoded = pipeline::transform_stream(&encoded, &decoder)?;
assert_eq!(input, decoded);
```

### Complex Template Processing

```rust
let complex_template = r#"
#SSX#
User: #V#username#V#
Message: #V#content#V#
Timestamp: #V#time#V#
#ESX#
Processing complete #EOF#
"#;

let encoder = transformers::extensions_encoder();
let input = format!("payload={}", complex_template);
let encoded = pipeline::transform_stream(&input, &encoder)?;

// Template structure is preserved through encoding/decoding
```

## Performance Considerations

### Compression Efficiency

ASC100 provides **87.5% bit efficiency** compared to standard 8-bit encoding:

```rust
let large_content = "Large dataset content ".repeat(1000);
let input = format!("data={}", large_content);

// Measure encoding performance
let start = std::time::Instant::now();
let encoded = pipeline::transform_stream(&input, &transformer)?;
let encode_time = start.elapsed();

println!("Original: {} bytes", input.len());
println!("Encoded: {} bytes", encoded.len());
println!("Compression: {:.1}%", (encoded.len() as f64 / input.len() as f64) * 100.0);
println!("Encode time: {:?}", encode_time);
```

### Selective Encoding for Performance

Only encode sensitive or large fields:

```rust
// Performance pattern: only encode specific fields
let sensitive_keys = &["password", "token", "key", "secret"];
let result = pipeline::transform_selective(
    input,
    &transformer,
    sensitive_keys
)?;
```

### Compression Gates

Use size-based encoding decisions:

```rust
// Only encode if content is larger than threshold
let min_size = 100; // bytes
let result = integration::compression_gate(input, &transformer, min_size)?;
```

## Real-World Examples

### Authentication Token Processing

```rust
use asc100::xstream_transformer::{transformers, pipeline};

fn process_auth_tokens(auth_data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let transformer = transformers::encoder_key();
    
    // Encode sensitive authentication data
    let encoded = pipeline::transform_selective(
        auth_data,
        &transformer,
        &["password", "token", "api_key", "secret"]
    )?;
    
    Ok(encoded)
}

// Usage
let auth = "user=john; password=secret123; token=xyz789; debug=true";
let encoded_auth = process_auth_tokens(auth)?;
println!("Safe auth: {}", encoded_auth);
// Output: "user=john; password_asc=c2VjcmV0MTIz; token_asc=eHl6Nzg5; debug=true"
```

### Configuration File Transmission

```rust
fn encode_config(config: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encoder = presets::core_key();
    
    // Encode entire configuration for safe transmission
    let encoded = utils::encode_token_string(config, &encoder)?;
    Ok(encoded)
}

fn decode_config(encoded_config: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encoder = presets::core_key();
    
    // Decode configuration on receiving end
    let decoded = utils::decode_token_string(encoded_config, &encoder)?;
    Ok(decoded)
}

// Usage
let config = "host=localhost; port=8080; ssl=true; path=/api/v1";
let encoded = encode_config(config)?;
let decoded = decode_config(&encoded)?;
assert_eq!(config, decoded);
```

### Template Processing Pipeline

```rust
use asc100::xstream_transformer::{transformers, pipeline};

fn process_template_pipeline(template: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encoder = transformers::extensions_encoder();
    
    // Step 1: Encode template with markers
    let encoded = pipeline::transform_stream(template, &encoder)?;
    
    // Step 2: Chain with additional processing
    let processed = pipeline::chain_transform(&encoded, &encoder, |intermediate| {
        // Add metadata
        Ok(format!("{}; processed_at={}; version=1.0", intermediate, 1234567890))
    })?;
    
    Ok(processed)
}

// Usage
let template = "template=Hello #V#name#V#, your order #EOF#";
let processed = process_template_pipeline(template)?;
println!("Processed: {}", processed);
```

### Bidirectional Stream Processing

```rust
fn smart_processor(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let transformer = transformers::bidirectional();
    
    // Automatically encodes unmarked content, decodes marked content
    let result = pipeline::transform_stream(input, &transformer)?;
    Ok(result)
}

// Mixed encoded/unencoded input
let mixed = "normal=text; encoded_asc=ZW5jb2RlZA==; new=content";
let processed = smart_processor(mixed)?;
println!("Smart processed: {}", processed);
// Output: "normal_asc=dGV4dA==; encoded=encoded; new_asc=Y29udGVudA=="
```

### Performance-Optimized Processing

```rust
fn optimized_processor(
    input: &str,
    sensitive_keys: &[&str],
    min_encode_size: usize
) -> Result<String, Box<dyn std::error::Error>> {
    let transformer = transformers::encoder_key();
    
    // Step 1: Selective encoding for sensitive fields
    let selective = pipeline::transform_selective(input, &transformer, sensitive_keys)?;
    
    // Step 2: Size-based encoding for remaining large content
    let optimized = integration::compression_gate(&selective, &transformer, min_encode_size)?;
    
    Ok(optimized)
}

// Usage
let data = "user=john; password=secret123; config=small; large_data={}";
let large_content = "x".repeat(1000);
let input = &data.replace("{}", &large_content);

let result = optimized_processor(
    input,
    &["password"],     // Always encode passwords
    100               // Encode content > 100 bytes
)?;
```

## API Reference

### Simple Integration Module (`xstream_simple`)

#### Core Types

```rust
pub enum Asc100Mode {
    KeySuffix,      // "key_asc=value"
    ValueSuffix,    // "key=value:a"
    Both,           // "key_asc=value:a"
}

pub struct Asc100ValueEncoder<S: EncodingStrategy> {
    // Core encoder functionality
}
```

#### Key Methods

```rust
impl<S: EncodingStrategy> Asc100ValueEncoder<S> {
    pub fn encode_value(&self, value: &str) -> Result<String, Asc100Error>;
    pub fn decode_value(&self, encoded_value: &str) -> Result<String, Asc100Error>;
    pub fn encode_kv_pair(&self, key: &str, value: &str) -> Result<(String, String), Asc100Error>;
    pub fn decode_kv_pair(&self, key: &str, value: &str) -> Result<(String, String), Asc100Error>;
}
```

#### Utility Functions

```rust
pub mod utils {
    pub fn encode_token_string<S: EncodingStrategy>(
        input: &str, 
        encoder: &Asc100ValueEncoder<S>
    ) -> Result<String, Box<dyn std::error::Error>>;
    
    pub fn decode_token_string<S: EncodingStrategy>(
        input: &str, 
        encoder: &Asc100ValueEncoder<S>
    ) -> Result<String, Box<dyn std::error::Error>>;
}
```

#### Presets

```rust
pub mod presets {
    pub fn core_key() -> Asc100ValueEncoder<CoreStrategy<StrictFilter>>;
    pub fn core_value() -> Asc100ValueEncoder<CoreStrategy<StrictFilter>>;
    pub fn extensions_key() -> Asc100ValueEncoder<ExtensionsStrategy<StrictFilter>>;
    pub fn extensions_both() -> Asc100ValueEncoder<ExtensionsStrategy<StrictFilter>>;
}
```

### Transformer Integration Module (`xstream_transformer`)

#### Transform Modes

```rust
pub enum TransformMode {
    EncodeKeyMarked,    // Encode with key suffix
    EncodeValueMarked,  // Encode with value suffix
    Decode,             // Decode marked content
    Bidirectional,      // Smart encode/decode
}
```

#### Pipeline Functions

```rust
pub mod pipeline {
    pub fn transform_stream<S: EncodingStrategy>(
        input: &str,
        transformer: &Asc100Transformer<S>
    ) -> Result<String, Box<dyn std::error::Error>>;
    
    pub fn chain_transform<S: EncodingStrategy, F>(
        input: &str,
        asc100_transformer: &Asc100Transformer<S>,
        next_operation: F
    ) -> Result<String, Box<dyn std::error::Error>>;
    
    pub fn transform_selective<S: EncodingStrategy>(
        input: &str,
        transformer: &Asc100Transformer<S>,
        key_filter: &[&str]
    ) -> Result<String, Box<dyn std::error::Error>>;
}
```

#### Integration Helpers

```rust
pub mod integration {
    pub fn compression_gate<S: EncodingStrategy>(
        input: &str,
        transformer: &Asc100Transformer<S>,
        min_size: usize
    ) -> Result<String, Box<dyn std::error::Error>>;
    
    pub fn fork_encode<S: EncodingStrategy>(
        input: &str,
        transformer: &Asc100Transformer<S>
    ) -> Result<(String, String), Box<dyn std::error::Error>>;
    
    pub fn merge_streams(
        encoded_stream: &str,
        decoded_stream: &str,
        prefer_encoded: bool
    ) -> Result<String, Box<dyn std::error::Error>>;
}
```

## Best Practices

### 1. Choose the Right Integration Approach
- Use **Simple Integration** for existing apps needing basic ASC100 encoding
- Use **Transformer Integration** for apps with XStream pipelines

### 2. Select Appropriate Encoding Modes
- **KeySuffix**: Best for debugging and transparency
- **ValueSuffix**: Best for minimal key changes
- **Bidirectional**: Best for mixed encoded/unencoded streams

### 3. Use Selective Encoding
- Only encode sensitive or large fields
- Use compression gates for size-based decisions
- Consider performance impact of encoding all fields

### 4. Template Strategy Selection
- Use **Core** strategy for simple text
- Use **Extensions** strategy when templates contain markers

### 5. Error Handling
Always handle encoding errors gracefully:

```rust
match encoder.encode_value(value) {
    Ok(encoded) => process_encoded(encoded),
    Err(Asc100Error::InvalidCharacter(ch)) => {
        log::warn!("Invalid character in value: {}", ch);
        handle_invalid_input()
    },
    Err(e) => return Err(e.into()),
}
```

## Testing

Run the comprehensive test suite:

```bash
# Run XStream integration tests
cargo test --features xstream test_xstream

# Run specific transformer tests
cargo test --features xstream transformer_tests

# Run performance tests
cargo test --features xstream test_large_content_performance
```

## Contributing

1. Fork the repository
2. Create a feature branch for XStream improvements
3. Add tests in `tests/test_xstream_*.rs`
4. Ensure all tests pass: `cargo test --features xstream`
5. Submit a pull request

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

For more information about ASC100 core functionality, see [README.md](README.md).
For implementation details and concepts, see [docs/CONCEPTS.md](docs/CONCEPTS.md).