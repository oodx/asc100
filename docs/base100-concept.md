# ASC100 - ASCII Super Compression System

## Overview

ASC100 (ASCII Super Compression 100) is a lossless binary encoding system designed for transmitting code, configuration files, and structured text through URL parameters and web APIs without data corruption. It solves the fundamental problem of safely transporting programming content through web infrastructure that has character limitations, while providing advanced features for protocol negotiation and structured data transmission.

## The Problem

Modern web applications frequently need to transmit code snippets, configuration data, or structured text through URLs (GET parameters, webhooks, API calls). However, standard approaches face several challenges:

- **URL Encoding Issues**: Special characters get percent-encoded (`%20`, `%22`, etc.), making URLs unwieldy
- **Character Set Limitations**: Many systems can't handle the full Unicode range
- **Data Corruption**: Copy/paste operations, email transmission, and legacy systems can mangle special characters
- **Size Inefficiency**: Base64 encoding everything wastes space for text that's mostly ASCII
- **Protocol Misalignment**: No way to verify that both endpoints understand the same format

## The Solution: ASC100 Encoding

ASC100 encoding creates a **collapsed character space** that includes exactly the characters needed for most programming and configuration tasks, then uses efficient binary encoding for transmission, enhanced with memo blocks for metadata and protocol negotiation.

### Core Concept

1. **Define a Base100 Character Set** (100 essential characters)
2. **Map input text** to this restricted set with extension markers
3. **Encode as 7-bit binary** (since 2^7 = 128 > 100)
4. **Pack binary efficiently** and output as URL-safe Base64
5. **Add memo blocks** for metadata and protocol verification

## Character Set Design

The ASC100 character set includes exactly 100 characters chosen for maximum programming utility:

### Standard ASCII Printable (95 characters)
```
 !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~
```
*Includes space (ASCII 32) through tilde (ASCII 126)*

### Essential Whitespace (4 characters)
- Tab (`\t`, ASCII 9) - Index ~95
- Newline (`\n`, ASCII 10) - Index ~96  
- Carriage Return (`\r`, ASCII 13) - Index ~97
- Null (`\0`, ASCII 0) - Index ~98

### Reserved Slot (1 character)
- Reserved for future base expansion - Index 99

**Total: 100 characters exactly (indices 0-99)**

## Extension System (Indices 100-127)

ASC100 reserves 28 extension slots for special markers and structured data:

### Core Extensions (100-108)
| Index | Marker | Purpose |
|-------|--------|---------|
| 100 | #SSX# | Start of stream marker |
| 101 | #ESX# | End of stream marker |
| 102 | #EOF# | End of file marker |
| 103 | #NL# | Explicit newline hint |
| 104 | #MEM# | Memo block delimiter |
| 105 | #HSO[key]# | Handshake Out - protocol definition |
| 106 | #HSI[key]# | Handshake In - protocol acknowledgment |
| 107 | #FX[name]FX# | Function wrapper |
| 108 | #ARG[data]ARG# | Argument wrapper |

### Custom Extensions (109-127)
19 slots available for application-specific markers.

## MEMO System

MEMO blocks provide metadata and configuration information using flat token format:

### Syntax
```
#MEM#[k1=v1;k2=v2;prefix:key=value;desc="quoted value"]#MEM#[content]
```

### Examples

**Simple version and checksum:**
```
#MEM#[v=1.0;c=a1b2c3d4]#MEM#VGhpcyBpcyBlbmNvZGVk
```

**With namespaced metadata:**
```
#MEM#[v=1.2;app:name=myapp;app:version=2.1;compression=gzip]#MEM#SGVsbG8...
```

**With quoted values:**
```
#MEM#[v=1.0;desc="JavaScript console output";meta:timestamp=1640995200]#MEM#...
```

### MEMO Rules
- **Plain text storage** - Just key=value pairs as strings
- **No magic processing** - ASC100 library only extracts, doesn't interpret
- **Application responsibility** - Receiving endpoint decides what to do with tokens
- **Flexible parsing** - Endpoints can handle quotes, spaces, namespacing as needed

## Handshake Protocol

The handshake system provides basic protocol alignment verification:

### Protocol Flow
```
// System A defines expected response
A → B: #MEM#[v=1.0]#MEM##HSO[session123]#VGVzdERhdGE...

// System B must echo the key back
B → A: #MEM#[v=1.0]#MEM##HSI[session123]#UmVzcG9uc2U...

// Mismatched or missing HSI = rejection
```

### Benefits
✅ **Prevents garbage data** - Unaligned systems won't know the key  
✅ **Simple verification** - Just string equality check  
✅ **No encryption** - Plain text matching for basic alignment  
✅ **Conversation tracking** - Each exchange can have unique keys

## Function System

ASC100 supports pseudo-code function calls for structured operations:

### Syntax
```
#FX[functionName]FX##ARG[parameter=value]ARG#
```

### Examples

**Simple function call:**
```
#FX[getUserData]FX##ARG[userId=123]ARG#
```

**Multiple arguments:**
```
#FX[calculateSum]FX##ARG[a=5]ARG##ARG[b=10]ARG#
```

**Embedded in code:**
```javascript
console.log('Starting...');
#FX[processData]FX##ARG[input=myfile.json]ARG#
console.log('Done.');
```

## Encoding Process

### Step 1: MEMO Processing
```
Input: #MEM#[v=1.0;c=a1b2c3d4]#MEM#console.log("Hello!");#EOF#
↓
MEMO: {v: "1.0", c: "a1b2c3d4"}
Content: console.log("Hello!");#EOF#
```

### Step 2: Extension Marker Preprocessing
```
Content: console.log("Hello!");#EOF#
↓
Processed: console.log("Hello!");[EOF_CHAR_102]
```

### Step 3: Character Index Mapping
```
'c' → 67, 'o' → 79, 'n' → 83, [EOF] → 102, ...
```

### Step 4: Binary Packing
```
67 → 1000011, 79 → 1001111, 102 → 1100110, ...
```

### Step 5: Base64 Output
```
Final: #MEM#[v=1.0;c=a1b2c3d4]#MEM#VVRGSDcaQiVAHUAhQiFAJEAwQCRAMEAkQDRANEAwQCRAMEAmQCxAI0AhQCNAYQ
```

## Key Properties

### Lossless Transmission
- **Perfect Fidelity**: Original text reconstructed exactly
- **No Data Loss**: Binary encoding preserves every character
- **Round-trip Verified**: Encode → Transmit → Decode = Original
- **Protocol Verification**: Handshake system ensures alignment

### URL Safety
- **Pure ASCII Output**: Only `A-Za-z0-9+/` characters
- **No Percent Encoding**: Works directly in query parameters
- **Standard Base64**: Compatible with all web infrastructure
- **Metadata Preserved**: MEMO blocks survive transmission

### Efficiency
- **87% Bit Efficiency**: 7 bits per character vs 8-bit ASCII
- **Compact Representation**: ~115% of input length vs ~133% for Base64
- **Smart Extensions**: 28 slots for structured data without base set bloat
- **Minimal Metadata**: MEMO blocks add minimal overhead

## Advanced Use Cases

### Protocol Negotiation
```
// Server defines protocol version and expects confirmation
Server: #MEM#[v=2.0;protocol=auth-v3]#MEM##HSO[session-abc123]#[auth_data]

// Client confirms understanding
Client: #MEM#[v=2.0]#MEM##HSI[session-abc123]##FX[authenticate]FX##ARG[token=xyz]ARG#
```

### RPC-Style Calls
```
// Remote function with structured arguments
#MEM#[v=1.0;target=dataservice]#MEM##FX[queryDatabase]FX##ARG[table=users]ARG##ARG[filter=active=true]ARG#
```

### Document Structure
```
// Structured document with metadata
#MEM#[v=1.0;type=code;lang=javascript]#MEM##SSX#
function main() {
    #FX[loadConfig]FX##ARG[env=production]ARG#
    #NL#console.log('Application started');
}#ESX##EOF#
```

## Implementation Architecture

### Modular Design
```
src/
├── lib.rs              // Main library exports
├── core/
│   ├── mod.rs          // Core module exports  
│   ├── encoder.rs      // Pure encode logic
│   ├── decoder.rs      // Pure decode logic
│   └── charset.rs      // Character set definitions
├── extensions/
│   ├── mod.rs          // Extension system
│   └── core_ext.rs     // Locked core extensions (100-108)
├── memo/
│   ├── mod.rs          // MEMO parsing utilities
│   └── parser.rs       // Token extraction helpers
└── error.rs            // Error types
```

### Separation of Concerns
- **Core Library**: Only handles encoding/decoding and extension marker extraction
- **MEMO Parser**: Utility functions for token parsing (application choice)
- **Protocol Handler**: Application implements handshake verification
- **Function System**: Application interprets FX/ARG blocks as needed

## Comparison with Alternatives

| Method | URL Safe | Efficiency | Fidelity | Protocol | Structured |
|--------|----------|------------|----------|----------|------------|
| Raw Text | ❌ | 100% | ❌ | ❌ | ❌ |
| URL Encoding | ✅ | ~60% | ✅ | ❌ | ❌ |
| Base64 | ✅ | 75% | ✅ | ❌ | ❌ |
| **ASC100** | ✅ | **87%** | ✅ | ✅ | ✅ |
| Custom Binary | ❌ | 90%+ | ✅ | ⚠ | ⚠ |

## Security Considerations

### Data Privacy
- **No Encryption**: ASC100 is encoding, not encryption
- **Visible Metadata**: MEMO blocks and handshakes are plain text
- **Transport Security**: Use HTTPS for sensitive data transmission

### Protocol Safety
- **Trust Classification**: TR/DNT markers provide content safety context
- **Input Validation**: Restrict to ASC100 character set prevents many injection attacks
- **Content Isolation**: Clear boundaries between trusted text and executable code
- **Structured Parsing**: FX system provides safer function call parsing

### Checksum Integrity
- **CRC32 Verification**: Optional checksum in MEMO blocks for corruption detection
- **Application Choice**: Endpoints decide whether to validate checksums
- **Transmission Safety**: Catches copy/paste errors and data corruption

## Future Extensions

### Enhanced Metadata
- **Digital Signatures**: Cryptographic verification in MEMO blocks
- **Compression Integration**: Pre-compression before ASC100 encoding
- **Content Types**: MIME-type hints for specialized parsing

### Protocol Evolution
- **Version Negotiation**: Automatic fallback for version mismatches
- **Capability Exchange**: Systems announce supported extensions
- **Backward Compatibility**: Graceful handling of unknown extensions

### Tooling Ecosystem
- **IDE Plugins**: Direct encode/decode in code editors
- **CLI Tools**: Command-line utilities for batch processing
- **Web Services**: Hosted encoding/decoding with MEMO parsing
- **Browser Extensions**: One-click encoding for web developers

## Conclusion

ASC100 encoding provides an optimal balance of **efficiency**, **safety**, **structure**, and **protocol awareness** for transmitting complex data through web infrastructure. It solves real-world problems faced by developers while providing a foundation for sophisticated application protocols.

The system's design prioritizes **practical utility** over theoretical optimization, making it immediately useful for common development tasks while enabling advanced features like protocol negotiation, structured function calls, and metadata transmission.

ASC100 transforms simple text encoding into a comprehensive communication protocol while maintaining perfect backward compatibility and URL safety.

---

*ASC100: Because your code deserves safe passage and smart protocols.*