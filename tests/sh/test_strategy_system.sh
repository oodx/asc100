#!/bin/bash

echo "Testing ASC100 Strategy System..."

# Quick test with the new strategy system
cat > /tmp/strategy_test.rs << 'EOF'
use asc100::char::extensions::{CoreStrategy, ExtensionsStrategy};
use asc100::char::versions::V1_STANDARD;
use asc100::{encode_with_strategy, decode_with_strategy};

fn main() {
    // Test 1: Core strategy (no extensions)
    let strategy = CoreStrategy::strict();
    let input = "Hello, World!";
    
    match encode_with_strategy(input, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy) {
        Ok(encoded) => {
            println!("Core encoded: {}", encoded);
            match decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy) {
                Ok(decoded) => {
                    if decoded == input {
                        println!("✓ Core strategy roundtrip successful!");
                    } else {
                        println!("✗ Core strategy failed: {} != {}", input, decoded);
                    }
                }
                Err(e) => println!("✗ Core decode error: {}", e),
            }
        }
        Err(e) => println!("✗ Core encode error: {}", e),
    }
    
    // Test 2: Extensions strategy with markers
    let strategy = ExtensionsStrategy::strict();
    let input_with_markers = "Hello #V#name#V# world #EOF#";
    
    match encode_with_strategy(input_with_markers, &V1_STANDARD.charset, &V1_STANDARD.lookup, &strategy) {
        Ok(encoded) => {
            println!("Extensions encoded: {}", encoded);
            match decode_with_strategy(&encoded, &V1_STANDARD.charset, &strategy) {
                Ok(decoded) => {
                    if decoded == input_with_markers {
                        println!("✓ Extensions strategy roundtrip successful!");
                    } else {
                        println!("✗ Extensions strategy failed: {} != {}", input_with_markers, decoded);
                    }
                }
                Err(e) => println!("✗ Extensions decode error: {}", e),
            }
        }
        Err(e) => println!("✗ Extensions encode error: {}", e),
    }
}
EOF

# Compile and run
rustc --edition 2021 -L target/debug/deps /tmp/strategy_test.rs -o /tmp/strategy_test --extern asc100=target/debug/libasc100.rlib 2>/dev/null
if [ $? -eq 0 ]; then
    /tmp/strategy_test
else
    echo "✗ Failed to compile strategy test"
    exit 1
fi

# Cleanup
rm -f /tmp/strategy_test*

echo "Strategy system test complete."