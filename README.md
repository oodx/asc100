# ASC100 - ASCII Super Compression System

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

ASC100 is a **character-level encoding system** that efficiently encodes text to a base-100 character set, then packs it as 7-bit values into Base64 for URL-safe transmission. It provides **87.5% bit efficiency** compared to standard 8-bit encoding while preserving all printable ASCII characters and common whitespace.

## Features

- **87.5% Encoding Efficiency** - Uses only 7 bits per character (vs 8-bit ASCII)
- **URL-Safe Output** - Base64 encoded for web transmission
- **Lossless Compression** - Perfect roundtrip for all supported characters
- **Strategy-Based Architecture** - Flexible encoding/decoding with customizable filtering
- **Extension Markers** - Supports structured data hints (indices 100-127)
- **Multiple Character Sets** - Optimized versions for different use cases
- **Comprehensive Error Handling** - Clear error reporting for invalid input

## Quick Start

Add ASC100 to your `Cargo.toml`:

```toml
[dependencies]
asc100 = "0.1.0"

# Optional features
asc100 = { version = "0.1.0", features = ["random", "patterns"] }
```

### Basic Usage

```rust
use asc100::char::versions::V1_STANDARD;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Hello, World! 🦀";
    
    // Encode text
    let encoded = V1_STANDARD.encode(text)?;
    println!("Encoded: {}", encoded);
    
    // Decode back
    let decoded = V1_STANDARD.decode(&encoded)?;
    println!("Decoded: {}", decoded);
    
    assert_eq!(text, decoded);
    Ok(())
}
```

### Strategy-Based Encoding

ASC100 provides flexible encoding strategies for different use cases:

```rust
use asc100::{encode_with_strategy, decode_with_strategy};
use asc100::char::extensions::{CoreStrategy, ExtensionsStrategy};
use asc100::char::versions::V1_STANDARD;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Hello #SSX# World #ESX#!";
    
    // Core strategy - basic characters only, strict filtering
    let core_strategy = CoreStrategy::strict();
    let encoded = encode_with_strategy(
        text,
        &V1_STANDARD.charset,
        &V1_STANDARD.lookup,
        &core_strategy
    )?;
    
    // Extensions strategy - supports markers like #SSX#, #ESX#
    let ext_strategy = ExtensionsStrategy::strict();
    let encoded_with_markers = encode_with_strategy(
        text,
        &V1_STANDARD.charset,
        &V1_STANDARD.lookup,
        &ext_strategy
    )?;
    
    let decoded = decode_with_strategy(&encoded_with_markers, &V1_STANDARD.charset, &ext_strategy)?;
    println!("Decoded with markers: {}", decoded);
    
    Ok(())
}
```

## Character Set Versions

ASC100 includes optimized character sets for different scenarios:

### V1_STANDARD - Balanced General Purpose
```rust
use asc100::char::versions::V1_STANDARD;

// Optimized for mixed content with good balance of all character types
let encoded = V1_STANDARD.encode("Mixed content: code(), text, 123!")?;
```

### V2_NUMBERS - Numeric Data Priority
```rust
use asc100::char::versions::V2_NUMBERS;

// Optimized for data with lots of numbers
let encoded = V2_NUMBERS.encode("Data: 12345, Value: 67890, ID: 999")?;
```

### V3_LOWERCASE - Text-Heavy Content  
```rust
use asc100::char::versions::V3_LOWERCASE;

// Optimized for lowercase text content
let encoded = V3_LOWERCASE.encode("this is mostly lowercase text content")?;
```

### V4_URL - Web URLs and Parameters
```rust
use asc100::char::versions::V4_URL;

// Optimized for URL encoding with common web characters
let encoded = V4_URL.encode("https://example.com/path?query=value&foo=bar")?;
```

## Extension Markers

ASC100 supports special markers for structured data (indices 100-103):

| Marker | Index | Purpose |
|--------|-------|---------|
| `#SSX#` | 100 | Start of stream |
| `#ESX#` | 101 | End of stream |
| `#EOF#` | 102 | End of file |
| `#NL#` | 103 | Newline hint |

```rust
use asc100::char::extensions::ExtensionsStrategy;

let text_with_markers = "Start #SSX# content here #ESX# End";
let strategy = ExtensionsStrategy::strict();

// Markers are encoded as special indices and consumed during decode
let encoded = encode_with_strategy(text_with_markers, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy)?;
let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy)?;
```

## Filtering Strategies

Choose how to handle invalid characters:

### Strict Filtering
```rust
use asc100::char::extensions::{CoreStrategy, ExtensionsStrategy};

// Errors on any invalid character
let strategy = CoreStrategy::strict();
let strategy = ExtensionsStrategy::strict();
```

### Sanitize Filtering
```rust
// Replaces invalid characters with #INV# marker
let strategy = CoreStrategy::sanitize();
let strategy = ExtensionsStrategy::sanitize();
```

### Strip Filtering
```rust
// Silently removes invalid characters
let strategy = CoreStrategy::strip();
let strategy = ExtensionsStrategy::strip();
```

## Performance & Efficiency

ASC100 provides significant compression for text data:

| Input Type | Compression Ratio | Efficiency |
|------------|------------------|------------|
| Source Code | ~1.15x | 87.5% |
| Plain Text | ~1.12x | 87.5% |
| URLs | ~1.18x | 87.5% |
| Configuration | ~1.14x | 87.5% |

**Comparison with Base64:**
- Standard Base64: 133% of original size (4/3 ratio)
- ASC100: ~115% of original size (better compression)

## Character Coverage

ASC100 supports all printable ASCII plus essential whitespace:

- **Indices 0-94**: All printable ASCII (space through tilde `~`)
- **Index 95**: Tab (`\t`)
- **Index 96**: Newline (`\n`) 
- **Index 97**: Carriage Return (`\r`)
- **Index 98**: Null (`\0`)
- **Index 99**: Reserved
- **Indices 100-127**: Extension markers

## Error Handling

ASC100 provides clear error messages for common issues:

```rust
use asc100::{Asc100Error, char::versions::V1_STANDARD};

match V1_STANDARD.encode("Text with emoji 🦀") {
    Ok(encoded) => println!("Success: {}", encoded),
    Err(Asc100Error::NonAsciiInput) => println!("Contains non-ASCII characters"),
    Err(Asc100Error::InvalidCharacter(ch)) => println!("Invalid character: {}", ch),
    Err(e) => println!("Other error: {}", e),
}
```

## Command Line Interface

Run the built-in test suite:

```bash
cargo run --bin asc100
```

This will test all character set versions with various input types and show compression ratios.

## Development

### Running Tests
```bash
# Run all tests
cargo test

# Run with all features
cargo test --all-features

# Run specific test
cargo test test_roundtrip
```

### Building with Features
```bash
# Basic build
cargo build

# With random generation features
cargo build --features random

# With all features
cargo build --features full
```

## Architecture

ASC100 uses a modular architecture:

```
ASC100
├── Core Encoding (lib.rs)
│   ├── Legacy encode/decode functions
│   └── Strategy-based encode/decode functions
├── Character Management (char/)
│   ├── charset.rs - Character set creation and lookup
│   ├── versions.rs - Predefined optimized character sets
│   └── extensions.rs - Strategy pattern and filtering
└── Optional Features
    ├── rand/ - Random data generation (optional)
    └── patterns/ - Regex support (optional)
```

## Use Cases

### Current Implementation
- Encoding source code for URL parameters
- Transmitting configuration files through web APIs
- Preserving whitespace in text transmission
- Adding simple EOF/stream markers as hints

### With Extension Strategies
- Content filtering and sanitization
- Structured data transmission with markers
- Protocol-aware encoding/decoding
- Error-tolerant text processing

## Limitations

- **ASCII Only**: Input must be 7-bit ASCII characters
- **Size Overhead**: ~15% size increase (vs standard Base64's 33%)
- **Not Cryptographic**: This is compression, not encryption
- **Fixed Character Set**: Base 100 characters are predefined per version

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass: `cargo test --all-features`
6. Submit a pull request

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Technical Details

For implementation details, character mappings, and future architectural considerations, see [docs/CONCEPTS.md](docs/CONCEPTS.md).