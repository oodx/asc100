# ASC100 - ASCII Super Compression System Concepts

## Core Concept

ASC100 is a **character-level encoding system** that maps text to a base-100 character set, then encodes it as 7-bit values packed into Base64 for URL-safe transmission. It provides 87.5% bit efficiency compared to standard 8-bit encoding.

**Key Features:**
- **Strategy-based encoding** with filtering for invalid characters
- **Extension markers** (indices 100-127) for structured data
- **Version compliance** via control markers
- **Cross-platform templating** with universal quote handling

## Architecture

### Character Mapping (Implemented)
```
Input Text → Character Indices (0-127) → 7-bit Binary → Base64 Output
```

### Key Design Decisions

1. **Base-100 Character Set (Indices 0-99)**
   - Indices 0-94: All printable ASCII (space through tilde)
   - Index 95: Tab (`\t`)
   - Index 96: Newline (`\n`)
   - Index 97: Carriage Return (`\r`)
   - Index 98: Null (`\0`)
   - Index 99: Reserved

2. **Extension Markers (Indices 100-103)**
   - 100: `#SSX#` - Start of stream
   - 101: `#ESX#` - End of stream
   - 102: `#EOF#` - End of file
   - 103: `#NL#` - Newline hint
   - **Indices 104-127**: Reserved for future extensions

## Current Implementation

ASC100 now supports **two encoding approaches**: legacy functions for backwards compatibility and a new **strategy-based system** for flexible encoding with filtering.

### Strategy-Based Architecture (New)

#### Encoding Strategies:
- **CoreStrategy**: Base 100 characters only (indices 0-99), no extension markers
- **ExtensionsStrategy**: Full support including markers (indices 100-127)

#### Filtering Options:
- **StrictFilter**: Errors on any invalid character
- **SanitizeFilter**: Replaces invalid characters with `#INV#` marker
- **StripFilter**: Silently removes invalid characters

#### Strategy-Based Encoding Process
1. **Preprocessing**: Apply filtering strategy + marker replacement (if Extensions strategy)
2. **Character Mapping**: Map each character to its index (0-99) or marker index (100-127)
3. **Binary Packing**: Convert indices to 7-bit binary values
4. **Base64 Encoding**: Pack binary into 6-bit chunks for Base64 output

#### Strategy-Based Decoding Process
1. **Base64 Decode**: Convert Base64 back to binary
2. **Index Extraction**: Extract 7-bit indices from binary
3. **Character/Marker Mapping**: Convert indices 0-99 to characters, 100-127 to marker strings
4. **Postprocessing**: Apply strategy postprocessing

### Legacy Implementation (Backwards Compatibility)

#### Legacy Encoding Process
1. **Marker Replacement**: Replace `#SSX#`, `#ESX#`, `#EOF#`, `#NL#` with temporary placeholders
2. **Character Mapping**: Map each character to its index (0-99) or marker index (100-103)
3. **Binary Packing**: Convert indices to 7-bit binary values
4. **Base64 Encoding**: Pack binary into 6-bit chunks for Base64 output

#### Legacy Decoding Process
1. **Base64 Decode**: Convert Base64 back to binary
2. **Index Extraction**: Extract 7-bit indices from binary
3. **Character Mapping**: Convert indices 0-99 back to characters
4. **Marker Handling**: Indices 100-103 are consumed as hints (no output)

### Important Behaviors

#### Strategy-Based System:
- **Marker Restoration**: Extension markers are restored as strings during decode (new behavior)
- **Flexible Filtering**: Invalid characters can be handled via error, replacement, or removal
- **Strategy Validation**: Decoders validate that indices are supported by the strategy
- **Whitespace Preservation**: Actual whitespace characters (`\n`, `\t`, ` `) are preserved
- **Index 0 is Space**: The space character maps to index 0 (not a null/padding indicator)

#### Legacy System:
- **One-Way Markers**: Extension markers encode to indices but don't regenerate on decode
- **Limited Extension Support**: Only supports indices 100-103
- **No Filtering**: Direct character validation without preprocessing options
- **Padding Handling**: Trailing zeros from bit padding are ignored during decode

## Future Considerations: Wrapper Patterns

While ASC100 is fundamentally a character-level encoder, future preprocessing layers could add structured data support:

### Potential Preprocessing Layer Architecture
```
Complex Syntax → Tokenizer → Simple Markers → ASC100 Encoder
                     ↓
              Application Layer
```

### Proposed Wrapper Patterns (Not Implemented)

#### Trust System
- `#TR[text]TR#` - Trusted content (human-readable, validated safe)
- `#DNT[code]DNT#` - Do Not Trust (code/executable content)

**Trust Validation Rules (Conceptual):**
- TR blocks would allow: letters, numbers, basic punctuation
- TR blocks would forbid: `< > [ ] { } ( ) + = * / % & | ^ ~ \``

#### Function System
- `#FX[name][args]FX#` - Function calls with integrated arguments
- Arguments use key=value pairs: `#FX[process][input=data.json;validate=true]FX#`

#### Metadata System
- `#MEM#[k1=v1;k2=v2]#MEM#` - Metadata blocks
- Supports namespacing: `app:version=1.0`
- Supports quoted values: `desc="Complex description"`

#### Protocol Handshake
- `#HSO[key]#` - Handshake out (expects response)
- `#HSI[key]#` - Handshake in (acknowledgment)

### Why These Are Future Considerations

1. **Scope Separation**: ASC100 is a character encoder, not a parser
2. **Variable Length**: Wrapper patterns have variable content that can't map to single indices
3. **Application Specific**: Different use cases need different preprocessing rules
4. **Complexity**: Would require tokenization, state management, and escape sequences

### Implementation Strategy for Wrappers

If implementing wrapper patterns, create a separate preprocessing layer:

```javascript
// Preprocessing Layer (Future)
function preprocessComplexPatterns(text) {
    // Tokenize #TR[...]TR# patterns
    // Validate content against rules
    // Convert to intermediate format
    // Pass to ASC100 encoder
}

// ASC100 Layer (Current)
function encodeASC100(text) {
    // Pure character-level encoding
    // Maps chars to indices 0-127
    // No pattern matching or parsing
}
```

## Design Principles

### What ASC100 IS
- ✅ Character-level encoder
- ✅ Lossless compression for URL transmission
- ✅ Simple marker replacement for hints
- ✅ 7-bit efficient encoding

### What ASC100 IS NOT
- ❌ Syntax parser
- ❌ Pattern matcher for complex expressions
- ❌ Security validator
- ❌ Markup language processor

## Use Cases

### Current (Implemented)
- **Basic Text Encoding**: Source code, configuration files, mixed content
- **URL-Safe Transmission**: Web APIs, query parameters, form data
- **Whitespace Preservation**: Code formatting, structured text
- **Content Filtering**: Sanitize or strip invalid characters automatically
- **Extension Markers**: Stream boundaries, file markers, protocol hints
- **Multiple Character Sets**: Optimized for numbers, lowercase text, URLs, general purpose

### Current Strategy Applications
- **Web Applications**: Sanitize user input while preserving valid content
- **Data Processing**: Strip problematic characters from imported text
- **Protocol Design**: Use extension markers for structured communication
- **Content Validation**: Strict filtering for security-sensitive applications

### Future (With Advanced Preprocessing Layer)
- Distinguishing trusted text from code
- Embedding function calls in transmitted data
- Protocol negotiation and handshakes
- Metadata transmission alongside content

## Technical Specifications

### Efficiency Metrics
- **Bit Usage**: 7 bits per character (vs 8-bit ASCII)
- **Efficiency**: 87.5% (7/8 bits)
- **Output Ratio**: ~1.15x input size (vs 1.33x for standard Base64)
- **Character Coverage**: 100 base chars + 28 extension indices

### Constraints
- **Maximum Index**: 127 (7-bit limit)
- **Base Character Set**: Fixed at 100 characters
- **Extension Space**: 28 indices (100-127)
- **Marker Format**: Simple text replacement only

## Implementation Notes

### Critical Details
1. **Space at Index 0**: Must handle index 0 as valid character (space), not padding
2. **Padding Detection**: Trailing zeros from bit padding must be distinguished from actual zero indices
3. **Marker Processing**: Markers are consumed during decode, not regenerated
4. **Binary Alignment**: 7-bit values must be properly padded to 6-bit boundaries for Base64

### Common Pitfalls
- Treating index 0 as null/padding instead of space
- Attempting to recreate markers on decode
- Trying to parse complex patterns at the character level
- Not handling the 7-bit to 6-bit conversion properly

## Implementation Status

### ✅ Phase 1: Core Encoding (Complete)
- Character-to-index mapping
- 7-bit binary packing
- Base64 output encoding
- Simple marker support
- Legacy API for backwards compatibility

### ✅ Phase 2: Strategy Architecture (Complete)
- Strategy-based encoding/decoding system
- Multiple filtering strategies (strict, sanitize, strip)
- Core vs Extensions strategy separation
- Flexible marker handling
- Comprehensive error handling

### ✅ Phase 3: Character Set Optimization (Complete)
- V1_STANDARD: Balanced general purpose
- V2_NUMBERS: Optimized for numeric data
- V3_LOWERCASE: Optimized for text content
- V4_URL: Optimized for web URLs

### 🔧 Known Issues
- **Extensions Strategy Marker Decoding**: Some marker restoration edge cases need refinement
- **Performance Optimization**: Strategy overhead could be reduced
- **Documentation**: API examples could be expanded

### 🚀 Future Enhancements
- Advanced preprocessing layer for complex patterns
- Trust validation system
- Function call parser
- Metadata extractor
- Streaming support
- Compression integration

## Conclusion

ASC100 succeeds as a focused character-level encoder. The wrapper patterns (#TR, #DNT, #FX, etc.) represent valuable future enhancements that would be implemented as a separate preprocessing layer on top of the core encoding system. This separation of concerns keeps the base encoder simple, reliable, and maintainable while allowing for future extensibility.