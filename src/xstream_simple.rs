//! Simple XStream integration for ASC100 encoding
//! 
//! This module provides value encoders that work with existing XStream tokens.
//! Instead of rebuilding the token system, this integrates as a value transformation layer.

use crate::char::extensions::{CoreStrategy, ExtensionsStrategy, EncodingStrategy};
use crate::char::versions::V1_STANDARD;
use crate::{encode_with_strategy, decode_with_strategy, Asc100Error};

/// Encoding mode for XStream token values
#[derive(Debug, Clone)]
pub enum Asc100Mode {
    /// Add `_asc` suffix to key: `content_asc="encoded_value"`
    KeySuffix,
    /// Add `:a` suffix to value: `content="encoded_value:a"`
    ValueSuffix,
    /// Use both approaches for maximum compatibility
    Both,
}

/// ASC100 value encoder for XStream tokens
pub struct Asc100ValueEncoder<S: EncodingStrategy> {
    strategy: S,
    mode: Asc100Mode,
}

impl Asc100ValueEncoder<CoreStrategy<crate::char::extensions::StrictFilter>> {
    /// Create encoder with Core strategy (basic encoding, no markers)
    pub fn core(mode: Asc100Mode) -> Self {
        Self {
            strategy: CoreStrategy::strict(),
            mode,
        }
    }
}

impl Asc100ValueEncoder<ExtensionsStrategy<crate::char::extensions::StrictFilter>> {
    /// Create encoder with Extensions strategy (supports markers)
    pub fn extensions(mode: Asc100Mode) -> Self {
        Self {
            strategy: ExtensionsStrategy::strict(),
            mode,
        }
    }
}

impl<S: EncodingStrategy> Asc100ValueEncoder<S> {
    /// Encode a value using ASC100
    pub fn encode_value(&self, value: &str) -> Result<String, Asc100Error> {
        encode_with_strategy(value, &V1_STANDARD.charset, &V1_STANDARD.lookup, &self.strategy)
    }

    /// Decode a value from ASC100
    pub fn decode_value(&self, encoded_value: &str) -> Result<String, Asc100Error> {
        // Remove suffix if present
        let clean_value = if encoded_value.ends_with(":a") {
            encoded_value.trim_end_matches(":a")
        } else {
            encoded_value
        };
        
        decode_with_strategy(clean_value, &V1_STANDARD.charset, &self.strategy)
    }

    /// Transform a key-value pair for encoding
    pub fn encode_kv_pair(&self, key: &str, value: &str) -> Result<(String, String), Asc100Error> {
        let encoded_value = self.encode_value(value)?;
        
        match self.mode {
            Asc100Mode::KeySuffix => Ok((format!("{}_asc", key), encoded_value)),
            Asc100Mode::ValueSuffix => Ok((key.to_string(), format!("{}:a", encoded_value))),
            Asc100Mode::Both => Ok((format!("{}_asc", key), format!("{}:a", encoded_value))),
        }
    }

    /// Transform a key-value pair for decoding (auto-detect encoding)
    pub fn decode_kv_pair(&self, key: &str, value: &str) -> Result<(String, String), Asc100Error> {
        let key_encoded = key.ends_with("_asc");
        let value_encoded = value.ends_with(":a");

        if !key_encoded && !value_encoded {
            // Not encoded, return as-is
            return Ok((key.to_string(), value.to_string()));
        }

        // Extract clean key and value
        let clean_key = if key_encoded {
            key.trim_end_matches("_asc")
        } else {
            key
        };
        
        let clean_encoded_value = if value_encoded {
            value.trim_end_matches(":a")
        } else {
            value
        };

        // Decode the value
        let decoded_value = self.decode_value(clean_encoded_value)?;
        Ok((clean_key.to_string(), decoded_value))
    }
}

/// Utility functions for working with token strings
pub mod utils {
    use super::*;

    /// Encode all values in a token string using ASC100
    pub fn encode_token_string<S: EncodingStrategy>(
        input: &str, 
        encoder: &Asc100ValueEncoder<S>
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut result_tokens = Vec::new();
        
        for token_str in input.split(';') {
            let token_str = token_str.trim();
            if token_str.is_empty() { continue; }
            
            // Parse key=value (supporting namespaces)
            let (key_part, value) = token_str.split_once('=')
                .ok_or_else(|| "Token must contain '='")?;
            
            let (encoded_key, encoded_value) = encoder.encode_kv_pair(key_part, value)?;
            result_tokens.push(format!("{}={}", encoded_key, encoded_value));
        }
        
        Ok(result_tokens.join("; "))
    }

    /// Decode all values in a token string from ASC100
    pub fn decode_token_string<S: EncodingStrategy>(
        input: &str, 
        encoder: &Asc100ValueEncoder<S>
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut result_tokens = Vec::new();
        
        for token_str in input.split(';') {
            let token_str = token_str.trim();
            if token_str.is_empty() { continue; }
            
            // Parse key=value
            let (key_part, value) = token_str.split_once('=')
                .ok_or_else(|| "Token must contain '='")?;
            
            let (clean_key, decoded_value) = encoder.decode_kv_pair(key_part, value)?;
            result_tokens.push(format!("{}={}", clean_key, decoded_value));
        }
        
        Ok(result_tokens.join("; "))
    }
}

/// Convenience presets for common use cases
pub mod presets {
    use super::*;

    /// Core strategy with key suffix encoding (recommended for basic use)
    pub fn core_key() -> Asc100ValueEncoder<CoreStrategy<crate::char::extensions::StrictFilter>> {
        Asc100ValueEncoder::core(Asc100Mode::KeySuffix)
    }

    /// Core strategy with value suffix encoding
    pub fn core_value() -> Asc100ValueEncoder<CoreStrategy<crate::char::extensions::StrictFilter>> {
        Asc100ValueEncoder::core(Asc100Mode::ValueSuffix)
    }

    /// Extensions strategy with key suffix (recommended for templates)
    pub fn extensions_key() -> Asc100ValueEncoder<ExtensionsStrategy<crate::char::extensions::StrictFilter>> {
        Asc100ValueEncoder::extensions(Asc100Mode::KeySuffix)
    }

    /// Extensions strategy with both indicators (maximum compatibility)
    pub fn extensions_both() -> Asc100ValueEncoder<ExtensionsStrategy<crate::char::extensions::StrictFilter>> {
        Asc100ValueEncoder::extensions(Asc100Mode::Both)
    }
}

/// Streamable wrapper for ASC100-encoded content
/// 
/// This allows any string to be automatically ASC100-encoded when used as a token value,
/// integrating with the XStream Streamable framework.
pub struct Asc100Streamable<S: EncodingStrategy> {
    pub content: String,
    pub encoder: Asc100ValueEncoder<S>,
}

impl<S: EncodingStrategy> Asc100Streamable<S> {
    pub fn new(content: String, encoder: Asc100ValueEncoder<S>) -> Self {
        Self { content, encoder }
    }
    
    /// Create with core strategy
    pub fn core(content: String, mode: Asc100Mode) -> Asc100Streamable<CoreStrategy<crate::char::extensions::StrictFilter>> {
        Asc100Streamable::new(content, Asc100ValueEncoder::core(mode))
    }
    
    /// Create with extensions strategy  
    pub fn extensions(content: String, mode: Asc100Mode) -> Asc100Streamable<ExtensionsStrategy<crate::char::extensions::StrictFilter>> {
        Asc100Streamable::new(content, Asc100ValueEncoder::extensions(mode))
    }
}

// Note: In a real integration, this would implement the actual XStream TokenStreamable trait
// impl<S: EncodingStrategy> TokenStreamable for Asc100Streamable<S> {
//     fn tokenize(&self) -> Result<Vec<Token>, String> {
//         // Encode the content and return as a single token value
//         let encoded = self.encoder.encode_value(&self.content)
//             .map_err(|e| e.to_string())?;
//         
//         let (key, value) = match self.encoder.mode {
//             Asc100Mode::KeySuffix => ("content_asc".to_string(), encoded),
//             Asc100Mode::ValueSuffix => ("content".to_string(), format!("{}:a", encoded)),
//             Asc100Mode::Both => ("content_asc".to_string(), format!("{}:a", encoded)),
//         };
//         
//         Ok(vec![Token { 
//             namespace: None, 
//             key, 
//             value 
//         }])
//     }
//     
//     fn validate(&self) -> Result<(), String> {
//         // Validate that content can be encoded
//         self.encoder.encode_value(&self.content)
//             .map(|_| ())
//             .map_err(|e| e.to_string())
//     }
// }