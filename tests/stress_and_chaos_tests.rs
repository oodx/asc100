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

#[test]
fn test_crazy_alternating_patterns() {
    // Test 1: Simple alternating high/low frequency pattern
    let mut pattern1 = String::new();
    let high_freq = ['e', 't', 'a', 'o', 'i']; 
    let low_freq = ['z', 'q', 'x', 'j', 'k'];
    for i in 0..200 {
        pattern1.push(if i % 2 == 0 { high_freq[i % high_freq.len()] } else { low_freq[i % low_freq.len()] });
    }
    let encoded1 = V1_STANDARD.encode(&pattern1).expect("Failed to encode frequency pattern");
    let decoded1 = V1_STANDARD.decode(&encoded1).expect("Failed to decode frequency pattern");
    assert_eq!(pattern1, decoded1, "Frequency alternating pattern failed");
    
    // Test 2: Class cycling (letters -> numbers -> symbols)
    let mut pattern2 = String::new();
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let numbers = "0123456789";
    let symbols = "!@#$%^&*()_+-=";
    for i in 0..150 {
        let ch = match i % 3 {
            0 => letters.chars().nth(i % letters.len()).unwrap(),
            1 => numbers.chars().nth(i % numbers.len()).unwrap(),
            _ => symbols.chars().nth(i % symbols.len()).unwrap(),
        };
        pattern2.push(ch);
    }
    let encoded2 = V1_STANDARD.encode(&pattern2).expect("Failed to encode class cycling");
    let decoded2 = V1_STANDARD.decode(&encoded2).expect("Failed to decode class cycling");
    assert_eq!(pattern2, decoded2, "Class cycling pattern failed");
    
    // Test 3: Character/space alternating
    let mut pattern3 = String::new();
    let chars = "HelloWorld123";
    for i in 0..100 {
        if i % 2 == 0 {
            pattern3.push(chars.chars().nth(i % chars.len()).unwrap());
        } else {
            pattern3.push(' ');
        }
    }
    let encoded3 = V1_STANDARD.encode(&pattern3).expect("Failed to encode alternating char/space");
    let decoded3 = V1_STANDARD.decode(&encoded3).expect("Failed to decode alternating char/space");
    assert_eq!(pattern3, decoded3, "Alternating char/space pattern failed");
}

#[test] 
fn test_pathological_edge_cases() {
    // Test 1: Worst-case bit patterns (extreme charset indices)
    let mut worst_case = String::new();
    let worst_chars = [
        V1_STANDARD.charset[0],   // Index 0: 0000000
        V1_STANDARD.charset[99],  // Index 99: 1100011
        V1_STANDARD.charset[63],  // Index 63: 0111111
        V1_STANDARD.charset[31],  // Index 31: 0011111
    ];
    for i in 0..200 {
        worst_case.push(worst_chars[i % worst_chars.len()]);
    }
    let encoded = V1_STANDARD.encode(&worst_case).expect("Failed to encode worst case");
    let decoded = V1_STANDARD.decode(&encoded).expect("Failed to decode worst case");
    assert_eq!(worst_case, decoded, "Worst case distribution failed");
    
    // Test 2: Compressibility nightmare (repetitive with variations)
    let mut nightmare = String::new();
    let base = "AaBbCc123!@#";
    for i in 0..50 {
        nightmare.push_str(base);
        if i % 7 == 0 { // Inject chaos every 7th iteration
            nightmare.push('X'); // Break the pattern
        }
    }
    let encoded = V1_STANDARD.encode(&nightmare).expect("Failed to encode nightmare");
    let decoded = V1_STANDARD.decode(&encoded).expect("Failed to decode nightmare");
    assert_eq!(nightmare, decoded, "Compressibility nightmare failed");
    
    // Test 3: Mixed control chars
    let mut control_mix = String::new();
    let controls = ['\t', '\n', '\r', '\0'];
    let normal = "Hello123";
    for i in 0..100 {
        if i % 5 == 0 {
            control_mix.push(controls[i % controls.len()]);
        } else {
            control_mix.push(normal.chars().nth(i % normal.len()).unwrap());
        }
    }
    let encoded = V1_STANDARD.encode(&control_mix).expect("Failed to encode control mix");
    let decoded = V1_STANDARD.decode(&encoded).expect("Failed to decode control mix");
    assert_eq!(control_mix, decoded, "Control char mix failed");
}

