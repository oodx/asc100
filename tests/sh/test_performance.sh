#!/bin/bash

# Performance test for ASC100 encoding

# Create test file with different sizes
sizes=(100 500 1000 5000)

for size in "${sizes[@]}"; do
    # Generate test data (repeated pattern to reach size)
    pattern="The quick brown fox jumps over the lazy dog. 1234567890!@#\$%^&*() "
    test_data=""
    while [ ${#test_data} -lt $size ]; do
        test_data="${test_data}${pattern}"
    done
    test_data="${test_data:0:$size}"  # Trim to exact size
    
    # Create temp test file
    echo "$test_data" > /tmp/perf_test_${size}.txt
    
    # Create Rust test program
    cat > /tmp/perf_test.rs << EOF
use asc100::versions::V1_STANDARD;

fn main() {
    let input = std::fs::read_to_string("/tmp/perf_test_${size}.txt").unwrap();
    let start = std::time::Instant::now();
    
    // Encode
    let encoded = V1_STANDARD.encode(&input).unwrap();
    let encode_time = start.elapsed();
    
    // Decode
    let decode_start = std::time::Instant::now();
    let decoded = V1_STANDARD.decode(&encoded).unwrap();
    let decode_time = decode_start.elapsed();
    
    // Verify
    assert_eq!(input, decoded);
    
    println!("Size: {} bytes | Encode: {:?} | Decode: {:?} | Ratio: {:.2}x",
             input.len(), encode_time, decode_time, 
             encoded.len() as f64 / input.len() as f64);
}
EOF
    
    # Compile and run
    rustc --edition 2021 -O -L target/release/deps /tmp/perf_test.rs -o /tmp/perf_test --extern asc100=target/release/libasc100.rlib 2>/dev/null
    /tmp/perf_test
done

# Cleanup
rm -f /tmp/perf_test* /tmp/perf_test.rs

# Success marker for test runner
exit 0