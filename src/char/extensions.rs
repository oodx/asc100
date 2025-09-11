// Marker processing is now handled in the main encoding logic

/// Action to take when encountering a character during filtering
#[derive(Debug, Clone)]
pub enum FilterAction {
    Keep,                    // Character is valid, keep it
    Replace(String),         // Replace with this string
    Skip,                   // Remove character silently  
    Error(char),            // Throw error for this character
}

/// Strategy for handling invalid characters during encoding
pub trait FilterStrategy {
    fn handle_char(&self, ch: char) -> FilterAction;
    
    /// Apply the filter strategy to the entire input string
    fn filter_input(&self, input: &str) -> Result<String, crate::Asc100Error> {
        let mut result = String::new();
        
        for ch in input.chars() {
            match self.handle_char(ch) {
                FilterAction::Keep => result.push(ch),
                FilterAction::Replace(replacement) => result.push_str(&replacement),
                FilterAction::Skip => {}, // Do nothing
                FilterAction::Error(invalid_char) => {
                    return Err(crate::Asc100Error::InvalidCharacter(invalid_char));
                }
            }
        }
        
        Ok(result)
    }
}

/// Strategy for handling encoding/decoding process
pub trait EncodingStrategy {
    fn preprocess(&self, input: &str) -> Result<String, crate::Asc100Error>;
    fn postprocess(&self, output: &str) -> String;
    fn supports_index(&self, index: u8) -> bool;
}

// ============================================================================
// FILTER STRATEGIES
// ============================================================================

/// Strict filter - errors on any invalid character
pub struct StrictFilter;

impl FilterStrategy for StrictFilter {
    fn handle_char(&self, ch: char) -> FilterAction {
        let ascii = ch as u32;
        if ascii < 128 && (ascii >= 32 && ascii <= 126 || matches!(ascii, 9 | 10 | 13 | 0 | 1)) {
            FilterAction::Keep
        } else {
            FilterAction::Error(ch)
        }
    }
}

/// Sanitize filter - replaces invalid characters with #INV# marker
pub struct SanitizeFilter;

impl FilterStrategy for SanitizeFilter {
    fn handle_char(&self, ch: char) -> FilterAction {
        let ascii = ch as u32;
        if ascii < 128 && (ascii >= 32 && ascii <= 126 || matches!(ascii, 9 | 10 | 13 | 0 | 1)) {
            FilterAction::Keep
        } else {
            FilterAction::Replace("#INV#".to_string())
        }
    }
}

/// Strip filter - removes invalid characters silently
pub struct StripFilter;

impl FilterStrategy for StripFilter {
    fn handle_char(&self, ch: char) -> FilterAction {
        let ascii = ch as u32;
        if ascii < 128 && (ascii >= 32 && ascii <= 126 || matches!(ascii, 9 | 10 | 13 | 0 | 1)) {
            FilterAction::Keep
        } else {
            FilterAction::Skip
        }
    }
}

// ============================================================================
// ENCODING STRATEGIES
// ============================================================================

/// Core strategy - base 100 characters only, no extensions
pub struct CoreStrategy<F: FilterStrategy> {
    pub filter: F,
}

impl<F: FilterStrategy> EncodingStrategy for CoreStrategy<F> {
    fn preprocess(&self, input: &str) -> Result<String, crate::Asc100Error> {
        // Only apply filter, no marker processing
        self.filter.filter_input(input)
    }
    
    fn postprocess(&self, output: &str) -> String {
        // No marker postprocessing
        output.to_string()
    }
    
    fn supports_index(&self, index: u8) -> bool {
        index < 100
    }
}

/// Extensions strategy - supports markers (100-127)
pub struct ExtensionsStrategy<F: FilterStrategy> {
    pub filter: F,
}

impl<F: FilterStrategy> EncodingStrategy for ExtensionsStrategy<F> {
    fn preprocess(&self, input: &str) -> Result<String, crate::Asc100Error> {
        // Only apply filter - markers are handled in tokenization phase
        self.filter.filter_input(input)
    }
    
    fn postprocess(&self, output: &str) -> String {
        // Markers are already restored during decode
        output.to_string()
    }
    
    fn supports_index(&self, index: u8) -> bool {
        index <= 127
    }
}

// ============================================================================
// CONVENIENCE CONSTRUCTORS
// ============================================================================

impl CoreStrategy<StrictFilter> {
    pub fn strict() -> Self {
        Self { filter: StrictFilter }
    }
}

impl CoreStrategy<SanitizeFilter> {
    pub fn sanitize() -> Self {
        Self { filter: SanitizeFilter }
    }
}

impl CoreStrategy<StripFilter> {
    pub fn strip() -> Self {
        Self { filter: StripFilter }
    }
}

impl ExtensionsStrategy<StrictFilter> {
    pub fn strict() -> Self {
        Self { filter: StrictFilter }
    }
}

impl ExtensionsStrategy<SanitizeFilter> {
    pub fn sanitize() -> Self {
        Self { filter: SanitizeFilter }
    }
}

impl ExtensionsStrategy<StripFilter> {
    pub fn strip() -> Self {
        Self { filter: StripFilter }
    }
}