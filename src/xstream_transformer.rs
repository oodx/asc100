//! ASC100 Transformer for XStream
//! 
//! Provides ASC100 encoding/decoding as XStream transformers that integrate
//! seamlessly with the existing XStream pipeline (adapters, merge, fork, gate, etc.)

use crate::char::extensions::{CoreStrategy, ExtensionsStrategy, EncodingStrategy};
use crate::char::versions::V1_STANDARD;
use crate::{encode_with_strategy, decode_with_strategy, Asc100Error};

/// ASC100 transformer modes for XStream integration
#[derive(Debug, Clone)]
pub enum TransformMode {
    /// Encode values using ASC100, add `:asc` to keys
    EncodeKeyMarked,
    /// Encode values using ASC100, add `:a` to values  
    EncodeValueMarked,
    /// Decode ASC100 values (auto-detect encoding markers)
    Decode,
    /// Bidirectional - encode unmarked, decode marked
    Bidirectional,
}

/// ASC100 transformer for XStream pipelines
pub struct Asc100Transformer<S: EncodingStrategy> {
    strategy: S,
    mode: TransformMode,
}

impl Asc100Transformer<CoreStrategy<crate::char::extensions::StrictFilter>> {
    /// Create transformer with Core strategy
    pub fn core(mode: TransformMode) -> Self {
        Self {
            strategy: CoreStrategy::strict(),
            mode,
        }
    }
}

impl Asc100Transformer<ExtensionsStrategy<crate::char::extensions::StrictFilter>> {
    /// Create transformer with Extensions strategy  
    pub fn extensions(mode: TransformMode) -> Self {
        Self {
            strategy: ExtensionsStrategy::strict(),
            mode,
        }
    }
}

impl<S: EncodingStrategy> Asc100Transformer<S> {
    /// Transform a token value according to the mode
    pub fn transform_value(&self, key: &str, value: &str) -> Result<(String, String), Asc100Error> {
        match self.mode {
            TransformMode::EncodeKeyMarked => {
                let encoded = self.encode_value(value)?;
                Ok((format!("{}_asc", key), encoded))
            }
            TransformMode::EncodeValueMarked => {
                let encoded = self.encode_value(value)?;
                Ok((key.to_string(), format!("{}:a", encoded)))
            }
            TransformMode::Decode => {
                self.try_decode_value(key, value)
            }
            TransformMode::Bidirectional => {
                // If already encoded, decode; if not encoded, encode
                if self.is_encoded(key, value) {
                    self.try_decode_value(key, value)
                } else {
                    let encoded = self.encode_value(value)?;
                    Ok((format!("{}_asc", key), encoded))
                }
            }
        }
    }

    /// Encode a value using ASC100
    fn encode_value(&self, value: &str) -> Result<String, Asc100Error> {
        encode_with_strategy(value, &V1_STANDARD.charset, &V1_STANDARD.lookup, &self.strategy)
    }

    /// Try to decode a value, return original if not encoded
    fn try_decode_value(&self, key: &str, value: &str) -> Result<(String, String), Asc100Error> {
        if !self.is_encoded(key, value) {
            return Ok((key.to_string(), value.to_string()));
        }

        let (clean_key, clean_value) = self.extract_encoded_parts(key, value);
        let decoded = decode_with_strategy(&clean_value, &V1_STANDARD.charset, &self.strategy)?;
        Ok((clean_key, decoded))
    }

    /// Check if a key-value pair is ASC100 encoded
    fn is_encoded(&self, key: &str, value: &str) -> bool {
        key.ends_with("_asc") || value.ends_with(":a")
    }

    /// Extract clean key and encoded value from marked pair
    fn extract_encoded_parts(&self, key: &str, value: &str) -> (String, String) {
        let clean_key = if key.ends_with("_asc") {
            key.trim_end_matches("_asc").to_string()
        } else {
            key.to_string()
        };

        let clean_value = if value.ends_with(":a") {
            value.trim_end_matches(":a").to_string()
        } else {
            value.to_string()
        };

        (clean_key, clean_value)
    }
}

/// XStream Pipeline Integration Functions
/// 
/// These functions are designed to integrate with XStream's transformer pipeline,
/// allowing ASC100 encoding/decoding to be composed with other XStream operations.
pub mod pipeline {
    use super::*;

    /// Transform a token stream string using ASC100
    pub fn transform_stream<S: EncodingStrategy>(
        input: &str,
        transformer: &Asc100Transformer<S>
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut result_tokens = Vec::new();
        
        for token_str in input.split(';') {
            let token_str = token_str.trim();
            if token_str.is_empty() { continue; }
            
            let (key_part, value) = token_str.split_once('=')
                .ok_or_else(|| "Token must contain '='")?;
            
            let (transformed_key, transformed_value) = transformer.transform_value(key_part, value)?;
            result_tokens.push(format!("{}={}", transformed_key, transformed_value));
        }
        
        Ok(result_tokens.join("; "))
    }

    /// Chain ASC100 transformation with other XStream operations
    /// 
    /// Example usage in XStream pipeline:
    /// input -> asc100_encode -> merge -> gate -> fork -> asc100_decode -> output
    pub fn chain_transform<S: EncodingStrategy, F>(
        input: &str,
        asc100_transformer: &Asc100Transformer<S>,
        next_operation: F
    ) -> Result<String, Box<dyn std::error::Error>>
    where
        F: Fn(&str) -> Result<String, Box<dyn std::error::Error>>
    {
        let asc100_result = transform_stream(input, asc100_transformer)?;
        next_operation(&asc100_result)
    }

    /// Apply ASC100 transformation only to specific keys (filter operation)
    pub fn transform_selective<S: EncodingStrategy>(
        input: &str,
        transformer: &Asc100Transformer<S>,
        key_filter: &[&str]
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut result_tokens = Vec::new();
        
        for token_str in input.split(';') {
            let token_str = token_str.trim();
            if token_str.is_empty() { continue; }
            
            let (key_part, value) = token_str.split_once('=')
                .ok_or_else(|| "Token must contain '='")?;
            
            // Extract namespace and key for filtering
            let actual_key = if let Some((_, k)) = key_part.split_once(':') {
                k
            } else {
                key_part
            };
            
            if key_filter.contains(&actual_key) {
                let (transformed_key, transformed_value) = transformer.transform_value(key_part, value)?;
                result_tokens.push(format!("{}={}", transformed_key, transformed_value));
            } else {
                result_tokens.push(token_str.to_string());
            }
        }
        
        Ok(result_tokens.join("; "))
    }
}

/// Predefined transformers for common XStream patterns
pub mod transformers {
    use super::*;

    /// Encoder transformer - adds :asc to keys
    pub fn encoder_key() -> Asc100Transformer<CoreStrategy<crate::char::extensions::StrictFilter>> {
        Asc100Transformer::core(TransformMode::EncodeKeyMarked)
    }

    /// Encoder transformer - adds :a to values
    pub fn encoder_value() -> Asc100Transformer<CoreStrategy<crate::char::extensions::StrictFilter>> {
        Asc100Transformer::core(TransformMode::EncodeValueMarked)
    }

    /// Decoder transformer - auto-detects and decodes
    pub fn decoder() -> Asc100Transformer<CoreStrategy<crate::char::extensions::StrictFilter>> {
        Asc100Transformer::core(TransformMode::Decode)
    }

    /// Bidirectional transformer - encodes unmarked, decodes marked
    pub fn bidirectional() -> Asc100Transformer<CoreStrategy<crate::char::extensions::StrictFilter>> {
        Asc100Transformer::core(TransformMode::Bidirectional)
    }

    /// Extensions encoder with template support
    pub fn extensions_encoder() -> Asc100Transformer<ExtensionsStrategy<crate::char::extensions::StrictFilter>> {
        Asc100Transformer::extensions(TransformMode::EncodeKeyMarked)
    }

    /// Extensions decoder with template support
    pub fn extensions_decoder() -> Asc100Transformer<ExtensionsStrategy<crate::char::extensions::StrictFilter>> {
        Asc100Transformer::extensions(TransformMode::Decode)
    }
}

/// Integration helpers for XStream ecosystem
pub mod integration {
    use super::*;

    /// Create a compression gate - only encode if content is large enough
    pub fn compression_gate<S: EncodingStrategy>(
        input: &str,
        transformer: &Asc100Transformer<S>,
        min_size: usize
    ) -> Result<String, Box<dyn std::error::Error>> {
        if input.len() >= min_size {
            pipeline::transform_stream(input, transformer)
        } else {
            Ok(input.to_string())
        }
    }

    /// Fork processing - encode one branch, leave other unchanged
    pub fn fork_encode<S: EncodingStrategy>(
        input: &str,
        transformer: &Asc100Transformer<S>
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        let encoded = pipeline::transform_stream(input, transformer)?;
        Ok((input.to_string(), encoded))
    }

    /// Merge encoded and decoded streams with conflict resolution
    pub fn merge_streams(
        encoded_stream: &str,
        decoded_stream: &str,
        prefer_encoded: bool
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Simple merge strategy - in real XStream this would be more sophisticated
        if prefer_encoded {
            Ok(encoded_stream.to_string())
        } else {
            Ok(decoded_stream.to_string())
        }
    }
}