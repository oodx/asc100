pub const fn create_base_charset() -> [char; 100] {
    let mut chars = ['\0'; 100];
    let mut idx = 0;
    
    // Standard ASCII printable (32-126): space through tilde
    let mut ascii = 32u8;
    while ascii <= 126 {
        chars[idx] = ascii as char;
        idx += 1;
        ascii += 1;
    }
    
    // Essential whitespace (95-99)
    chars[95] = '\t';
    chars[96] = '\n';
    chars[97] = '\r';
    chars[98] = '\0';
    chars[99] = '\x01';  // Reserved
    
    chars
}

pub const fn swap_chars(mut chars: [char; 100], idx1: usize, idx2: usize) -> [char; 100] {
    let temp = chars[idx1];
    chars[idx1] = chars[idx2];
    chars[idx2] = temp;
    chars
}

pub const fn swap_ranges(
    mut chars: [char; 100], 
    r1_start: usize, 
    r1_len: usize,
    r2_start: usize, 
    r2_len: usize
) -> [char; 100] {
    if r1_len != r2_len {
        panic!("Range lengths must match");
    }
    
    let mut i = 0;
    while i < r1_len {
        let temp = chars[r1_start + i];
        chars[r1_start + i] = chars[r2_start + i];
        chars[r2_start + i] = temp;
        i += 1;
    }
    chars
}

pub const fn build_lookup_table(charset: [char; 100]) -> [u8; 128] {
    let mut table = [255u8; 128];
    let mut i = 0;
    
    while i < 100 {
        let ch = charset[i];
        let ascii = ch as u32;
        if ascii < 128 {
            table[ascii as usize] = i as u8;
        }
        i += 1;
    }
    
    table
}

pub const BASE64_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
];

pub const fn build_base64_lookup() -> [u8; 128] {
    let mut table = [255u8; 128];
    let mut i = 0;
    
    while i < 64 {
        let ch = BASE64_CHARS[i];
        table[ch as usize] = i as u8;
        i += 1;
    }
    
    table
}

pub const BASE64_LOOKUP: [u8; 128] = build_base64_lookup();

// Extension markers (indices 100-127)
pub const MARKER_INV: u8 = 100;  // Invalid character placeholder
pub const MARKER_EOF: u8 = 101;  // End of file  
pub const MARKER_NL: u8 = 102;   // Newline hint
pub const MARKER_V: u8 = 103;    // Variable placeholder (shortened from VAR)
pub const MARKER_Q: u8 = 104;    // Double quote "
pub const MARKER_E: u8 = 105;    // Escape/single quote  
pub const MARKER_X: u8 = 106;    // Control/validation marker (V1 specific)

pub const MARKER_SSX: u8 = 107;  // Start stream
pub const MARKER_ESX: u8 = 108;  // End stream

pub const MARKER_MEM: u8 = 109;  // Encoding/transmission metadata
pub const MARKER_CTX: u8 = 110;  // Content/payload context

pub const MARKER_FX: u8 = 111;   // Function/code block
pub const MARKER_ARG: u8 = 112;  // Arguments/parameters
pub const MARKER_TR: u8 = 113;   // Trusted content
pub const MARKER_DNT: u8 = 114;  // Do not trust
pub const MARKER_BRK: u8 = 115;  // Break/separator

pub const MARKER_HSO: u8 = 116;  // Handshake out
pub const MARKER_HSI: u8 = 117;  // Handshake in
pub const MARKER_ACK: u8 = 118;  // Acknowledge

// Reserved markers: 119-127 (9 slots available)
// Note: Future versions should use different indices for MARKER_X

// Marker strings for preprocessing
pub const MARKERS: &[(&str, u8)] = &[
    ("#INV#", MARKER_INV),
    ("#EOF#", MARKER_EOF),
    ("#NL#", MARKER_NL),
    ("#V#", MARKER_V),
    ("#Q#", MARKER_Q),
    ("#E#", MARKER_E),
    ("#X#", MARKER_X),
    ("#SSX#", MARKER_SSX),
    ("#ESX#", MARKER_ESX),
    ("#MEM#", MARKER_MEM),
    ("#CTX#", MARKER_CTX),
    ("#FX#", MARKER_FX),
    ("#ARG#", MARKER_ARG),
    ("#TR#", MARKER_TR),
    ("#DNT#", MARKER_DNT),
    ("#BRK#", MARKER_BRK),
    ("#HSO#", MARKER_HSO),
    ("#HSI#", MARKER_HSI),
    ("#ACK#", MARKER_ACK),
];

/// Replace marker strings with their corresponding byte values for encoding
pub fn preprocess_markers(text: &str) -> String {
    let mut result = text.to_string();
    
    // Sort by length (longest first) to avoid substring conflicts
    let mut sorted_markers = MARKERS.to_vec();
    sorted_markers.sort_by_key(|(marker, _)| std::cmp::Reverse(marker.len()));
    
    for (marker_str, marker_index) in sorted_markers {
        let replacement_char = char::from(marker_index);
        result = result.replace(marker_str, &replacement_char.to_string());
    }
    
    result
}

/// Restore marker byte values back to their string representations for decoding
pub fn postprocess_markers(text: &str) -> String {
    let mut result = text.to_string();
    
    for (marker_str, marker_index) in MARKERS {
        let marker_char = char::from(*marker_index);
        result = result.replace(&marker_char.to_string(), marker_str);
    }
    
    result
}

/// Check if a byte value is a valid extension marker
pub const fn is_extension_marker(index: u8) -> bool {
    index >= 100 && index <= 127
}