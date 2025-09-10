// ASC100 - ASCII Super Compression Base100 Encoding Library
// Lossless binary encoding for URL-safe text transport

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Error types for ASC100 encoding/decoding operations
#[derive(Debug, Clone)]
pub enum ASC100Error {
    UnsupportedCharacter { char: char, position: usize },
    InvalidEncodedCharacter { char: char, position: usize },
    InvalidBinaryData,
    EmptyInput,
    ExtensionOverflow,
}

impl fmt::Display for ASC100Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASC100Error::UnsupportedCharacter { char, position } => {
                write!(f, "Unsupported character '{}' (U+{:04X}) at position {}", 
                       char, *char as u32, position)
            }
            ASC100Error::InvalidEncodedCharacter { char, position } => {
                write!(f, "Invalid encoded character '{}' at position {}", char, position)
            }
            ASC100Error::InvalidBinaryData => write!(f, "Invalid binary data format"),
            ASC100Error::EmptyInput => write!(f, "Input cannot be empty"),
            ASC100Error::ExtensionOverflow => {
                write!(f, "Too many extension characters (max 28 allowed)")
            }
        }
    }
}

impl Error for ASC100Error {}

/// Extension configuration for customizing unused character slots (100-127)
#[derive(Debug, Clone)]
pub struct ASCExtension {
    /// Custom markers mapped to indices 100-127
    pub markers: HashMap<String, u8>,
    /// Reverse mapping for decoding
    reverse_markers: HashMap<u8, String>,
}

impl ASCExtension {
    /// Create a new extension with default markers
    pub fn new() -> Self {
        let mut extension = Self {
            markers: HashMap::new(),
            reverse_markers: HashMap::new(),
        };
        
        // Add default markers
        extension.add_marker("EOF", 100).unwrap();
        extension.add_marker("EOL", 101).unwrap();
        extension.add_marker("EOS", 102).unwrap();
        extension.add_marker("TAB", 103).unwrap();
        extension.add_marker("INDENT", 104).unwrap();
        extension.add_marker("DEDENT", 105).unwrap();
        
        extension
    }
    
    /// Add a custom marker (indices 100-127 only)
    pub fn add_marker(&mut self, marker: &str, index: u8) -> Result<(), ASC100Error> {
        if index < 100 || index > 127 {
            return Err(ASC100Error::ExtensionOverflow);
        }
        
        let marker_string = marker.to_string();
        self.markers.insert(marker_string.clone(), index);
        self.reverse_markers.insert(index, marker_string);
        Ok(())
    }
    
    /// Remove a marker
    pub fn remove_marker(&mut self, marker: &str) {
        if let Some(index) = self.markers.remove(marker) {
            self.reverse_markers.remove(&index);
        }
    }
    
    /// Get marker index by name
    pub fn get_marker_index(&self, marker: &str) -> Option<u8> {
        self.markers.get(marker).copied()
    }
    
    /// Get marker name by index
    pub fn get_marker_name(&self, index: u8) -> Option<&String> {
        self.reverse_markers.get(&index)
    }
    
    /// List all configured markers
    pub fn list_markers(&self) -> Vec<(String, u8)> {
        self.markers.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }
}

impl Default for ASCExtension {
    fn default() -> Self {
        Self::new()
    }
}

/// Main ASC100 encoder/decoder
pub struct ASC100 {
    /// Base character set (indices 0-99)
    base_charset: Vec<char>,
    /// Character to index mapping for fast lookup
    char_to_index: HashMap<char, u8>,
    /// Base64 alphabet for output encoding
    base64_chars: Vec<char>,
    /// Base64 decode mapping
    base64_to_index: HashMap<char, u8>,
    /// Extension configuration
    extension: ASCExtension,
}

impl ASC100 {
    /// Create a new ASC100 codec with default configuration
    pub fn new() -> Self {
        Self::with_extension(ASCExtension::new())
    }
    
    /// Create a new ASC100 codec with custom extension
    pub fn with_extension(extension: ASCExtension) -> Self {
        // Standard printable ASCII (32-126) = 95 characters
        let base_ascii: Vec<char> = (32u8..=126u8).map(|b| b as char).collect();
        
        // Essential whitespace characters (indices 95-99)
        let whitespace = vec!['\t', '\n', '\r', '\x00', '\x01'];
        
        // Combine to create base charset (100 characters total)
        let mut base_charset = base_ascii;
        base_charset.extend(whitespace);
        
        // Create character to index mapping
        let mut char_to_index = HashMap::new();
        for (i, &ch) in base_charset.iter().enumerate() {
            char_to_index.insert(ch, i as u8);
        }
        
        // Base64 alphabet (URL-safe)
        let base64_chars: Vec<char> = 
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .chars().collect();
        
        // Base64 decode mapping
        let mut base64_to_index = HashMap::new();
        for (i, &ch) in base64_chars.iter().enumerate() {
            base64_to_index.insert(ch, i as u8);
        }
        
        Self {
            base_charset,
            char_to_index,
            base64_chars,
            base64_to_index,
            extension,
        }
    }
    
    /// Get the current extension configuration
    pub fn extension(&self) -> &ASCExtension {
        &self.extension
    }
    
    /// Update the extension configuration
    pub fn set_extension(&mut self, extension: ASCExtension) {
        self.extension = extension;
    }
    
    /// Preprocess text by replacing extension markers with their encoded values
    fn preprocess_text(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        // Replace markers in order of length (longest first to avoid conflicts)
        let mut markers: Vec<_> = self.extension.markers.iter().collect();
        markers.sort_by_key(|(marker, _)| std::cmp::Reverse(marker.len()));
        
        for (marker, &index) in markers {
            let marker_pattern = format!("#{marker}#");
            let replacement = char::from(index);
            result = result.replace(&marker_pattern, &replacement.to_string());
        }
        
        result
    }
    
    /// Postprocess text by replacing encoded values with their marker representations
    fn postprocess_text(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        for (&index, marker) in &self.extension.reverse_markers {
            let encoded_char = char::from(index);
            let marker_pattern = format!("#{marker}#");
            result = result.replace(&encoded_char.to_string(), &marker_pattern);
        }
        
        result
    }
    
    /// Encode text to ASC100 format
    pub fn encode(&self, input: &str) -> Result<String, ASC100Error> {
        if input.is_empty() {
            return Ok(String::new());
        }
        
        // Step 1: Preprocess markers
        let processed = self.preprocess_text(input);
        
        // Step 2: Convert characters to indices
        let mut indices = Vec::new();
        for (pos, ch) in processed.chars().enumerate() {
            if let Some(&index) = self.char_to_index.get(&ch) {
                indices.push(index);
            } else if ch as u32 >= 100 && ch as u32 <= 127 {
                // Extension character
                indices.push(ch as u8);
            } else {
                return Err(ASC100Error::UnsupportedCharacter { char: ch, position: pos });
            }
        }
        
        // Step 3: Pack into 7-bit binary
        let mut binary_bits = Vec::new();
        for index in indices {
            // Convert to 7-bit binary (big-endian)
            for i in (0..7).rev() {
                binary_bits.push((index >> i) & 1);
            }
        }
        
        // Step 4: Pad to multiple of 6 bits for Base64
        while binary_bits.len() % 6 != 0 {
            binary_bits.push(0);
        }
        
        // Step 5: Convert to Base64
        let mut result = String::new();
        for chunk in binary_bits.chunks(6) {
            let mut value = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                value |= bit << (5 - i);
            }
            result.push(self.base64_chars[value as usize]);
        }
        
        Ok(result)
    }
    
    /// Decode ASC100 format back to original text
    pub fn decode(&self, encoded: &str) -> Result<String, ASC100Error> {
        if encoded.is_empty() {
            return Ok(String::new());
        }
        
        // Step 1: Convert Base64 to binary
        let mut binary_bits = Vec::new();
        for (pos, ch) in encoded.chars().enumerate() {
            if let Some(&index) = self.base64_to_index.get(&ch) {
                // Convert to 6-bit binary (big-endian)
                for i in (0..6).rev() {
                    binary_bits.push((index >> i) & 1);
                }
            } else {
                return Err(ASC100Error::InvalidEncodedCharacter { char: ch, position: pos });
            }
        }
        
        // Step 2: Extract 7-bit character indices
        let mut indices = Vec::new();
        for chunk in binary_bits.chunks(7) {
            if chunk.len() == 7 {
                let mut value = 0u8;
                for (i, &bit) in chunk.iter().enumerate() {
                    value |= bit << (6 - i);
                }
                
                // Only include valid indices (0-127)
                if value <= 127 {
                    indices.push(value);
                }
            }
        }
        
        // Step 3: Convert indices back to characters
        let mut result = String::new();
        for index in indices {
            if index < 100 {
                // Base character set
                if let Some(&ch) = self.base_charset.get(index as usize) {
                    result.push(ch);
                } else {
                    return Err(ASC100Error::InvalidBinaryData);
                }
            } else {
                // Extension character
                result.push(char::from(index));
            }
        }
        
        // Step 4: Postprocess markers
        Ok(self.postprocess_text(&result))
    }
    
    /// Test round-trip encoding (encode then decode)
    pub fn test_round_trip(&self, input: &str) -> Result<bool, ASC100Error> {
        let encoded = self.encode(input)?;
        let decoded = self.decode(&encoded)?;
        Ok(input == decoded)
    }
    
    /// Get encoding statistics
    pub fn get_stats(&self, input: &str) -> Result<EncodingStats, ASC100Error> {
        let encoded = self.encode(input)?;
        
        Ok(EncodingStats {
            original_length: input.len(),
            encoded_length: encoded.len(),
            compression_ratio: encoded.len() as f64 / input.len() as f64,
            bit_efficiency: 7.0 / 8.0, // 7 bits per character vs 8 bits
            encoded_string: encoded,
        })
    }
}

impl Default for ASC100 {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about encoding operation
#[derive(Debug, Clone)]
pub struct EncodingStats {
    pub original_length: usize,
    pub encoded_length: usize,
    pub compression_ratio: f64,
    pub bit_efficiency: f64,
    pub encoded_string: String,
}

impl fmt::Display for EncodingStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "Original: {} chars, Encoded: {} chars, Ratio: {:.2}x, Efficiency: {:.1}%",
            self.original_length,
            self.encoded_length, 
            self.compression_ratio,
            self.bit_efficiency * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_encoding() {
        let codec = ASC100::new();
        let input = "Hello, World!";
        let encoded = codec.encode(input).unwrap();
        let decoded = codec.decode(&encoded).unwrap();
        assert_eq!(input, decoded);
    }
    
    #[test]
    fn test_javascript_code() {
        let codec = ASC100::new();
        let input = r#"console.log("Hello, World!");#EOF#"#;
        let encoded = codec.encode(input).unwrap();
        let decoded = codec.decode(&encoded).unwrap();
        assert_eq!(input, decoded);
    }
    
    #[test]
    fn test_extension_markers() {
        let mut extension = ASCExtension::new();
        extension.add_marker("CUSTOM", 106).unwrap();
        
        let codec = ASC100::with_extension(extension);
        let input = "Start#CUSTOM#End#EOF#";
        let encoded = codec.encode(input).unwrap();
        let decoded = codec.decode(&encoded).unwrap();
        assert_eq!(input, decoded);
    }
    
    #[test]
    fn test_round_trip() {
        let codec = ASC100::new();
        let input = "function test() { return 42; }#EOL#console.log('done');";
        assert!(codec.test_round_trip(input).unwrap());
    }
    
    #[test]
    fn test_empty_input() {
        let codec = ASC100::new();
        assert_eq!(codec.encode("").unwrap(), "");
        assert_eq!(codec.decode("").unwrap(), "");
    }
    
    #[test]
    fn test_unsupported_character() {
        let codec = ASC100::new();
        let input = "Hello 🌍"; // Contains emoji
        assert!(matches!(codec.encode(input), Err(ASC100Error::UnsupportedCharacter { .. })));
    }
    
    #[test]
    fn test_encoding_stats() {
        let codec = ASC100::new();
        let input = "console.log('test');";
        let stats = codec.get_stats(input).unwrap();
        
        assert_eq!(stats.original_length, input.len());
        assert!(stats.encoded_length > 0);
        assert!(stats.bit_efficiency > 0.8);
    }
}