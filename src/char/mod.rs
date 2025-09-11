pub mod charset;
pub mod versions;
pub mod extensions;

// Re-export commonly used items
pub use charset::{
    create_base_charset,
    swap_chars,
    swap_ranges,
    build_lookup_table,
    BASE64_CHARS,
    BASE64_LOOKUP,
    preprocess_markers,
    postprocess_markers,
    MARKERS,
};