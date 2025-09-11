pub mod char;

#[cfg(feature = "random")]
pub mod rand;

#[cfg(feature = "xstream")]
pub mod xstream_simple;

#[cfg(feature = "xstream")]
pub mod xstream_transformer;

use char::{BASE64_CHARS, BASE64_LOOKUP, preprocess_markers, postprocess_markers, MARKERS};
use char::extensions::EncodingStrategy;

// Sentinel-based representation for two-phase encoding
#[derive(Debug, Clone)]
enum Sentinel {
    Text(String),
    Marker(u8),
}

// Re-export commonly used items from char module
pub use char::versions;

#[derive(Debug, Clone)]
pub enum Asc100Error {
    InvalidCharacter(char),
    InvalidBase64Character(char),
    InvalidIndex(u8),
    NonAsciiInput,
}

impl std::fmt::Display for Asc100Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Asc100Error::InvalidCharacter(c) => write!(f, "Invalid character: '{}'", c),
            Asc100Error::InvalidBase64Character(c) => write!(f, "Invalid base64 character: '{}'", c),
            Asc100Error::InvalidIndex(i) => write!(f, "Invalid index: {}", i),
            Asc100Error::NonAsciiInput => write!(f, "Input contains non-ASCII characters"),
        }
    }
}

impl std::error::Error for Asc100Error {}

// ============================================================================
// TWO-PHASE TOKENIZATION
// ============================================================================

/// Parse input into sentinels, separating text from markers
fn parse_sentinels<S: EncodingStrategy>(input: &str, strategy: &S) -> Result<Vec<Sentinel>, Asc100Error> {
    let mut sentinels = Vec::new();
    let mut current_text = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '#' {
            // Potential marker start
            let mut marker_candidate = String::from("#");
            
            // Collect characters until next #
            while let Some(&next_ch) = chars.peek() {
                marker_candidate.push(chars.next().unwrap());
                if next_ch == '#' {
                    break;
                }
            }
            
            // Check if this is a valid marker
            if let Some((_, marker_index)) = MARKERS.iter().find(|(marker_str, _)| *marker_str == &marker_candidate) {
                if strategy.supports_index(*marker_index) {
                    // Valid marker - save any accumulated text first
                    if !current_text.is_empty() {
                        sentinels.push(Sentinel::Text(current_text.clone()));
                        current_text.clear();
                    }
                    sentinels.push(Sentinel::Marker(*marker_index));
                    continue;
                }
            }
            
            // Not a valid marker, treat as regular text
            current_text.push_str(&marker_candidate);
        } else {
            current_text.push(ch);
        }
    }
    
    // Add any remaining text
    if !current_text.is_empty() {
        sentinels.push(Sentinel::Text(current_text));
    }
    
    Ok(sentinels)
}

// ============================================================================
// STRATEGY-BASED ENCODING (NEW)
// ============================================================================

pub fn encode_with_strategy<S: EncodingStrategy>(
    input: &str, 
    _charset: &[char; 100], 
    lookup: &[u8; 128], 
    strategy: &S
) -> Result<String, Asc100Error> {
    // Phase 1: Apply strategy preprocessing (filtering only)
    let filtered_input = strategy.preprocess(input)?;
    
    // Phase 2: Parse into sentinels (text and markers)
    let sentinels = parse_sentinels(&filtered_input, strategy)?;
    
    let mut indices = Vec::new();
    
    // Phase 3: Convert sentinels to indices
    for sentinel in sentinels {
        match sentinel {
            Sentinel::Text(text) => {
                // Convert text characters to charset indices
                for ch in text.chars() {
                    let ascii = ch as u32;
                    if ascii >= 128 {
                        return Err(Asc100Error::NonAsciiInput);
                    }
                    
                    let index = lookup[ascii as usize];
                    if index == 255 {
                        return Err(Asc100Error::InvalidCharacter(ch));
                    }
                    indices.push(index);
                }
            }
            Sentinel::Marker(marker_index) => {
                // Use marker index directly
                indices.push(marker_index);
            }
        }
    }
    
    // Convert indices to 7-bit binary
    let mut bits = Vec::with_capacity(indices.len() * 7);
    for index in indices {
        for i in (0..7).rev() {
            bits.push((index >> i) & 1);
        }
    }
    
    // Pad to multiple of 6 for base64
    while bits.len() % 6 != 0 {
        bits.push(0);
    }
    
    // Pack into base64
    let mut result = String::with_capacity((bits.len() / 6) + 1);
    for chunk in bits.chunks(6) {
        let mut value = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            value |= bit << (5 - i);
        }
        result.push(BASE64_CHARS[value as usize]);
    }
    
    Ok(result)
}

pub fn decode_with_strategy<S: EncodingStrategy>(
    encoded: &str, 
    charset: &[char; 100], 
    strategy: &S
) -> Result<String, Asc100Error> {
    // Convert base64 to binary
    let mut bits = Vec::with_capacity(encoded.len() * 6);
    
    for ch in encoded.chars() {
        let ascii = ch as u32;
        if ascii >= 128 {
            return Err(Asc100Error::InvalidBase64Character(ch));
        }
        
        let value = char::BASE64_LOOKUP[ascii as usize];
        if value == 255 {
            return Err(Asc100Error::InvalidBase64Character(ch));
        }
        
        for i in (0..6).rev() {
            bits.push((value >> i) & 1);
        }
    }
    
    // Extract 7-bit indices
    let mut indices = Vec::new();
    for chunk in bits.chunks(7) {
        if chunk.len() == 7 {
            let mut index = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                index |= bit << (6 - i);
            }
            
            if index <= 127 {
                indices.push(index);
            }
        }
    }
    
    // Convert indices to characters
    let mut result = String::with_capacity(indices.len());
    for index in indices {
        if index >= 100 && index <= 127 {
            // Extension marker - check if strategy supports it
            if !strategy.supports_index(index) {
                return Err(Asc100Error::InvalidIndex(index));
            }
            // Convert marker index directly to marker string
            let marker_str = MARKERS.iter()
                .find(|(_, marker_index)| *marker_index == index)
                .map(|(marker_str, _)| *marker_str)
                .unwrap_or("");
            result.push_str(marker_str);
        } else if index < 100 {
            // Regular character from charset
            result.push(charset[index as usize]);
        } else {
            return Err(Asc100Error::InvalidIndex(index));
        }
    }
    
    // Apply strategy postprocessing
    Ok(strategy.postprocess(&result))
}

// ============================================================================
// LEGACY ENCODING (BACKWARDS COMPATIBILITY)
// ============================================================================

pub fn encode(input: &str, _charset: &[char; 100], lookup: &[u8; 128]) -> Result<String, Asc100Error> {
    // Step 1: Preprocess markers
    let processed_input = preprocess_markers(input);
    
    let mut indices = Vec::with_capacity(processed_input.len());
    
    // Convert characters to indices
    for ch in processed_input.chars() {
        let ascii = ch as u32;
        if ascii >= 128 {
            return Err(Asc100Error::NonAsciiInput);
        }
        
        let index = if ascii >= 100 && ascii <= 127 {
            // Extension marker - use directly
            ascii as u8
        } else {
            // Regular character - look up in charset
            let idx = lookup[ascii as usize];
            if idx == 255 {
                return Err(Asc100Error::InvalidCharacter(ch));
            }
            idx
        };
        
        indices.push(index);
    }
    
    // Convert indices to 7-bit binary
    let mut bits = Vec::with_capacity(indices.len() * 7);
    for index in indices {
        for i in (0..7).rev() {
            bits.push((index >> i) & 1);
        }
    }
    
    // Pad to multiple of 6 for base64
    while bits.len() % 6 != 0 {
        bits.push(0);
    }
    
    // Pack into base64
    let mut result = String::with_capacity((bits.len() / 6) + 1);
    for chunk in bits.chunks(6) {
        let mut value = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            value |= bit << (5 - i);
        }
        result.push(BASE64_CHARS[value as usize]);
    }
    
    Ok(result)
}

pub fn decode(encoded: &str, charset: &[char; 100]) -> Result<String, Asc100Error> {
    // Convert base64 to binary
    let mut bits = Vec::with_capacity(encoded.len() * 6);
    
    for ch in encoded.chars() {
        let ascii = ch as u32;
        if ascii >= 128 {
            return Err(Asc100Error::InvalidBase64Character(ch));
        }
        
        let value = BASE64_LOOKUP[ascii as usize];
        if value == 255 {
            return Err(Asc100Error::InvalidBase64Character(ch));
        }
        
        for i in (0..6).rev() {
            bits.push((value >> i) & 1);
        }
    }
    
    // Extract 7-bit indices
    let mut indices = Vec::new();
    for chunk in bits.chunks(7) {
        if chunk.len() == 7 {
            let mut index = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                index |= bit << (6 - i);
            }
            
            if index < 100 {
                indices.push(index);
            }
        }
    }
    
    // Convert indices to characters
    let mut result = String::with_capacity(indices.len());
    for index in indices {
        if index >= 100 && index <= 127 {
            // Extension marker - convert back to char
            result.push(char::from(index));
        } else if index < 100 {
            // Regular character from charset
            result.push(charset[index as usize]);
        } else {
            return Err(Asc100Error::InvalidIndex(index));
        }
    }
    
    // Step 4: Postprocess markers
    Ok(postprocess_markers(&result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::versions::V1_STANDARD;
    use crate::char::extensions::CoreStrategy;
    
    #[test]
    fn test_roundtrip() {
        let test_cases = vec![
            "Hello, World!",
            "1234567890",
            "~!@#$%^&*()_+",
            "The quick brown fox jumps over the lazy dog",
            "\t\n\r",
            " ",
            "~",
        ];
        
        let strategy = CoreStrategy::strict();
        
        for input in test_cases {
            let encoded = encode_with_strategy(input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy).unwrap();
            let decoded = decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy).unwrap();
            assert_eq!(input, decoded, "Roundtrip failed for: {}", input);
        }
    }
}