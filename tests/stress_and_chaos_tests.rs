use asc100::char::versions::V1_STANDARD;

#[cfg(feature = "random")]
use asc100::rand::{get_rand_string, get_rand_from_slice, rand_range_usize};

#[test]
fn test_insane_whitespace_stress() {
    // Pre-allocate long strings to avoid lifetime issues
    let spaces_100 = " ".repeat(100);
    let spaces_1000 = " ".repeat(1000);
    let boundary_test = format!("{}{}{}", " ".repeat(50), "content", " ".repeat(50));
    
    let test_cases = vec![
        // Single spaces
        (" ", "single space"),
        ("  ", "double space"),
        ("   ", "triple space"),
        ("          ", "ten spaces"),
        
        // Nothing but spaces
        (spaces_100.as_str(), "100 spaces"),
        (spaces_1000.as_str(), "1000 spaces"),
        
        // Mixed whitespace
        ("\t\n\r ", "tab-newline-return-space"),
        ("\t\t\t\t\t", "five tabs"),
        ("\n\n\n\n\n", "five newlines"),
        ("\r\r\r\r\r", "five returns"),
        
        // Whitespace sandwiches
        ("   hello   ", "spaces around hello"),
        ("\t\ndata\r\n\t", "whitespace sandwich"),
        
        // Edge cases
        ("", "empty string"),
        ("\0", "null character"),
        ("~", "tilde (index 0 in V1)"),
        
        // Boundary stress
        (boundary_test.as_str(), "space boundary test"),
    ];
    
    for (test_input, description) in test_cases {
        let encoded = V1_STANDARD.encode(&test_input).expect(&format!("Failed to encode {}", description));
        let decoded = V1_STANDARD.decode(&encoded).expect(&format!("Failed to decode {}", description));
        assert_eq!(test_input, decoded, "Roundtrip failed for {}: expected {:?}, got {:?}", description, test_input, decoded);
    }
}

#[test]
fn test_crazy_marker_stress() {
    // Pre-allocate marker stress strings
    let v_markers_100 = "#V#".repeat(100);
    let eof_markers_50 = "#EOF#".repeat(50);
    let huge_marker_text = format!("{}#V#{}", "x".repeat(1000), "y".repeat(1000));
    
    let marker_tests = vec![
        // Basic markers
        ("#V#", "basic variable marker"),
        ("#EOF#", "end of file marker"),
        ("#SSX#", "start stream marker"),
        ("#ESX#", "end stream marker"),
        
        // Fake/invalid markers (should be treated as text)
        ("#X#", "fake X marker"),
        ("#FAKE#", "fake long marker"),
        ("#", "incomplete marker"),
        ("##", "double hash"),
        ("#V", "incomplete V marker"),
        ("V#", "backwards V marker"),
        
        // Marker combinations
        ("#V##EOF#", "adjacent markers"),
        ("#V# #EOF#", "markers with space"),
        ("text#V#more#EOF#end", "text with markers"),
        
        // Marker stress tests
        (v_markers_100.as_str(), "100 V markers"),
        (eof_markers_50.as_str(), "50 EOF markers"),
        (huge_marker_text.as_str(), "markers in huge text"),
        
        // Edge cases with markers
        ("#V#", "lone V marker"),
        ("", "empty (no markers)"),
        ("no markers here", "text without markers"),
        ("##V##", "malformed marker attempts"),
    ];
    
    for (test_input, description) in marker_tests {
        let encoded = V1_STANDARD.encode(&test_input).expect(&format!("Failed to encode {}", description));
        let decoded = V1_STANDARD.decode(&encoded).expect(&format!("Failed to decode {}", description));
        assert_eq!(test_input, decoded, "Roundtrip failed for {}: expected {:?}, got {:?}", description, test_input, decoded);
    }
}

#[cfg(feature = "random")]
#[test]
fn test_crazy_alternating_patterns() {
    // Test 1: Character frequency alternating pattern (high freq <-> low freq)
    let pattern1 = generate_frequency_alternating_string(1000);
    let encoded1 = V1_STANDARD.encode(&pattern1).expect("Failed to encode frequency pattern");
    let decoded1 = V1_STANDARD.decode(&encoded1).expect("Failed to decode frequency pattern");
    assert_eq!(pattern1, decoded1, "Frequency alternating pattern failed");
    
    // Test 2: Character class cycling (letters -> numbers -> symbols)
    let pattern2 = generate_class_cycling_string(500);
    let encoded2 = V1_STANDARD.encode(&pattern2).expect("Failed to encode class cycling");
    let decoded2 = V1_STANDARD.decode(&encoded2).expect("Failed to decode class cycling");
    assert_eq!(pattern2, decoded2, "Class cycling pattern failed");
    
    // Test 3: Alternating character/space pattern
    let pattern3 = generate_alternating_char_space(200);
    let encoded3 = V1_STANDARD.encode(&pattern3).expect("Failed to encode alternating char/space");
    let decoded3 = V1_STANDARD.decode(&encoded3).expect("Failed to decode alternating char/space");
    assert_eq!(pattern3, decoded3, "Alternating char/space pattern failed");
    
    // Test 4: Random charset index hopping 
    let pattern4 = generate_charset_index_hopping(300);
    let encoded4 = V1_STANDARD.encode(&pattern4).expect("Failed to encode index hopping");
    let decoded4 = V1_STANDARD.decode(&encoded4).expect("Failed to decode index hopping");
    assert_eq!(pattern4, decoded4, "Charset index hopping pattern failed");
    
    // Test 5: Marker injection in random patterns
    let pattern5 = generate_random_with_markers(400);
    let encoded5 = V1_STANDARD.encode(&pattern5).expect("Failed to encode random with markers");
    let decoded5 = V1_STANDARD.decode(&encoded5).expect("Failed to decode random with markers");
    assert_eq!(pattern5, decoded5, "Random with markers pattern failed");
}

#[cfg(feature = "random")]
#[test] 
fn test_pathological_edge_cases() {
    // Pathological Case 1: Worst-case charset distribution
    let worst_case = generate_worst_case_distribution(500);
    let encoded = V1_STANDARD.encode(&worst_case).expect("Failed to encode worst case");
    let decoded = V1_STANDARD.decode(&encoded).expect("Failed to decode worst case");
    assert_eq!(worst_case, decoded, "Worst case distribution failed");
    
    // Pathological Case 2: Maximum entropy string  
    let max_entropy = generate_max_entropy_string(200);
    let encoded = V1_STANDARD.encode(&max_entropy).expect("Failed to encode max entropy");
    let decoded = V1_STANDARD.decode(&encoded).expect("Failed to decode max entropy");
    assert_eq!(max_entropy, decoded, "Max entropy string failed");
    
    // Pathological Case 3: Compressibility nightmare (highly repetitive but complex pattern)
    let nightmare = generate_compressibility_nightmare(300);
    let encoded = V1_STANDARD.encode(&nightmare).expect("Failed to encode nightmare");
    let decoded = V1_STANDARD.decode(&encoded).expect("Failed to decode nightmare");
    assert_eq!(nightmare, decoded, "Compressibility nightmare failed");
}

#[cfg(feature = "random")]
#[test]
fn test_unicode_boundary_stress() {
    // Test strings that push ASCII boundaries but stay valid
    let boundary_tests = vec![
        generate_ascii_boundary_string(100), // Characters near ASCII 127
        generate_control_char_mix(150),      // Mix of control characters
        generate_printable_spectrum(200),    // Full printable ASCII spectrum
    ];
    
    for (i, test_string) in boundary_tests.iter().enumerate() {
        let encoded = V1_STANDARD.encode(test_string).expect(&format!("Failed to encode boundary test {}", i));
        let decoded = V1_STANDARD.decode(&encoded).expect(&format!("Failed to decode boundary test {}", i));
        assert_eq!(*test_string, decoded, "Boundary test {} failed", i);
    }
}

#[cfg(feature = "random")]
mod chaos_generators {
    use super::*;
    
    pub fn generate_frequency_alternating_string(length: usize) -> String {
        // Alternate between high-frequency and low-frequency characters
        let high_freq = ['e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r']; // Common English letters
        let low_freq = ['z', 'q', 'x', 'j', 'k', 'v', 'b', 'p', 'y', 'w'];
        
        let mut result = String::with_capacity(length);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        
        for i in 0..length {
            let ch = if i % 2 == 0 {
                *high_freq.choose(&mut rng).unwrap()
            } else {
                *low_freq.choose(&mut rng).unwrap()
            };
            result.push(ch);
        }
        result
    }
    
    pub fn generate_class_cycling_string(length: usize) -> String {
        let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let numbers = "0123456789";
        let symbols = "!@#$%^&*()_+-=[]{}|;:'\",.<>?/~`";
        
        let mut result = String::with_capacity(length);
        let mut rng = ChaCha8Rng::seed_from_u64(123);
        
        for i in 0..length {
            let ch = match i % 3 {
                0 => letters.chars().choose(&mut rng).unwrap(),
                1 => numbers.chars().choose(&mut rng).unwrap(),
                _ => symbols.chars().choose(&mut rng).unwrap(),
            };
            result.push(ch);
        }
        result
    }
    
    pub fn generate_alternating_char_space(length: usize) -> String {
        let printable = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
        let mut result = String::with_capacity(length * 2);
        let mut rng = ChaCha8Rng::seed_from_u64(456);
        
        for i in 0..length {
            if i % 2 == 0 {
                result.push(printable.chars().choose(&mut rng).unwrap());
            } else {
                result.push(' ');
            }
        }
        result
    }
    
    pub fn generate_charset_index_hopping(length: usize) -> String {
        // Jump around the charset indices in a chaotic pattern
        let mut result = String::with_capacity(length);
        let mut rng = ChaCha8Rng::seed_from_u64(789);
        
        for _ in 0..length {
            // Pick random index from charset
            let index = rng.gen_range(0..100);
            result.push(V1_STANDARD.charset[index]);
        }
        result
    }
    
    pub fn generate_random_with_markers(length: usize) -> String {
        let normal_chars = "abcdefghijklmnopqrstuvwxyz0123456789 .,!?";
        let markers = ["#V#", "#EOF#", "#SSX#", "#ESX#"];
        
        let mut result = String::new();
        let mut rng = ChaCha8Rng::seed_from_u64(999);
        let mut chars_added = 0;
        
        while chars_added < length {
            if rng.gen::<f32>() < 0.1 && chars_added > 0 { // 10% chance of marker
                let marker = markers.choose(&mut rng).unwrap();
                result.push_str(marker);
                chars_added += marker.len();
            } else {
                result.push(normal_chars.chars().choose(&mut rng).unwrap());
                chars_added += 1;
            }
        }
        
        result.truncate(length);
        result
    }
    
    pub fn generate_worst_case_distribution(length: usize) -> String {
        // Use characters that are at opposite ends of the charset for worst bit patterns
        let worst_chars = [
            V1_STANDARD.charset[0],   // Index 0: 0000000
            V1_STANDARD.charset[99],  // Index 99: 1100011
            V1_STANDARD.charset[63],  // Index 63: 0111111
            V1_STANDARD.charset[31],  // Index 31: 0011111
        ];
        
        let mut result = String::with_capacity(length);
        let mut rng = ChaCha8Rng::seed_from_u64(1337);
        
        for _ in 0..length {
            result.push(*worst_chars.choose(&mut rng).unwrap());
        }
        result
    }
    
    pub fn generate_max_entropy_string(length: usize) -> String {
        // Generate string with maximum randomness across full charset
        let mut result = String::with_capacity(length);
        let mut rng = ChaCha8Rng::seed_from_u64(2468);
        
        for _ in 0..length {
            let index = rng.gen_range(0..100);
            result.push(V1_STANDARD.charset[index]);
        }
        result
    }
    
    pub fn generate_compressibility_nightmare(length: usize) -> String {
        // Pattern that looks repetitive but has subtle variations that kill compression
        let base_pattern = "AaBbCc123!@#";
        let mut result = String::with_capacity(length);
        let mut rng = ChaCha8Rng::seed_from_u64(1234);
        
        let mut pos = 0;
        while result.len() < length {
            let mut pattern = base_pattern.to_string();
            
            // Inject random variations to break compression
            if rng.gen::<f32>() < 0.3 {
                let insert_pos = rng.gen_range(0..pattern.len());
                let random_char = V1_STANDARD.charset[rng.gen_range(0..100)];
                pattern.insert(insert_pos, random_char);
            }
            
            result.push_str(&pattern);
            pos += 1;
        }
        
        result.truncate(length);
        result
    }
    
    pub fn generate_ascii_boundary_string(length: usize) -> String {
        // Test characters near ASCII boundaries
        let boundary_chars = [
            '\x20', '\x21', '\x7E', '\x7F', // Space, !, ~, DEL
            '\x00', '\x01', '\x02', '\x03', // Control chars
            '\x30', '\x39', '\x41', '\x5A', // Numbers, caps
            '\x61', '\x7A',                  // Lowercase
        ];
        
        let mut result = String::with_capacity(length);
        let mut rng = ChaCha8Rng::seed_from_u64(3698);
        
        for _ in 0..length {
            // Pick boundary chars that are valid in our charset
            let ch = loop {
                let candidate = *boundary_chars.choose(&mut rng).unwrap();
                if V1_STANDARD.lookup[candidate as usize] != 255 {
                    break candidate;
                }
            };
            result.push(ch);
        }
        result
    }
    
    pub fn generate_control_char_mix(length: usize) -> String {
        let control_chars = ['\t', '\n', '\r', '\0'];
        let normal_chars = "Hello World 123";
        
        let mut result = String::with_capacity(length);
        let mut rng = ChaCha8Rng::seed_from_u64(9876);
        
        for _ in 0..length {
            let ch = if rng.gen::<f32>() < 0.2 {
                *control_chars.choose(&mut rng).unwrap()
            } else {
                normal_chars.chars().choose(&mut rng).unwrap()
            };
            result.push(ch);
        }
        result
    }
    
    pub fn generate_printable_spectrum(length: usize) -> String {
        // Use full range of printable ASCII (32-126)
        let mut result = String::with_capacity(length);
        let mut rng = ChaCha8Rng::seed_from_u64(5555);
        
        for _ in 0..length {
            // Generate printable ASCII and check if it's in our charset
            let ch = loop {
                let ascii_val = rng.gen_range(32..=126) as u8;
                let candidate = ascii_val as char;
                if V1_STANDARD.lookup[candidate as usize] != 255 {
                    break candidate;
                }
            };
            result.push(ch);
        }
        result
    }
}

// Re-export the generators for use in tests
#[cfg(feature = "random")]
use chaos_generators::*;