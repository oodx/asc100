pub mod random;
pub mod gen;

// Re-export commonly used functions
pub use random::{
    get_rand_alnum,
    get_rand_alpha,
    get_rand_hex,
    get_rand_string,
    get_rand_uuid,
    get_rand_from_slice,
    rand_range_usize,
};

pub use gen::{
    gen_token,
    gen_flat_token,
    gen_ns_token,
    gen_token_stream,
    gen_config_stream,
    gen_stream_lines,
    gen_timed_stream,
    gen_log_stream,
    ValueType,
};