# ASC100 System Analysis

## Core Concept
ASC100 is a character-level encoding system that maps text to a base-100 character set, then encodes as 7-bit values packed into Base64. Key aspects:

- **87.5% bit efficiency** (7-bit vs 8-bit encoding)
- **URL-safe transmission** via Base64 output
- **100 base characters** (indices 0-99): 95 printable ASCII + 5 essential whitespace
- **28 extension slots** (indices 100-127) for markers and protocol features

## Current Implementation Status
- Working JavaScript implementation (HTML demos)
- Rust library reference with core encoding/decoding
- Comprehensive documentation covering concepts and future extensions

## Architecture Layers
1. **Core Encoding Layer** - Pure character-to-index mapping and 7-bit packing
2. **Extension System** - Markers mapped to indices 100-127
3. **Protocol Layer** (future) - MEMO blocks, handshakes, function calls

## Key Technical Considerations
1. **Index 0 = Space** - Must handle properly, not as null/padding
2. **Padding Detection** - Distinguish trailing zeros from actual zero indices
3. **Binary Alignment** - 7-bit to 6-bit conversion for Base64
4. **One-way markers** - Consumed during decode, not regenerated

## Future Module Structure
```
asc100/
├── src/
│   ├── lib.rs              // Public API
│   ├── core/
│   │   ├── charset.rs      // Character mappings
│   │   ├── encoder.rs      // Encoding logic
│   │   └── decoder.rs      // Decoding logic
│   ├── extensions/
│   │   ├── markers.rs      // Extension markers
│   │   └── registry.rs     // Extension registry
│   ├── protocol/
│   │   ├── memo.rs         // MEMO block parser
│   │   ├── handshake.rs    // HSO/HSI protocol
│   │   └── functions.rs    // FX/ARG system
│   └── error.rs            // Error types
├── benches/
│   └── encoding.rs         // Performance benchmarks
└── examples/
    ├── cli.rs              // CLI tool
    └── streaming.rs        // Stream processing
```

## Phase Plan (Original)
1. Core Library - Base100 encoding/decoding
2. Extension System - Markers and slots
3. Protocol Features - MEMO, handshakes, functions
4. Tools & Integration - CLI, benchmarks, streaming