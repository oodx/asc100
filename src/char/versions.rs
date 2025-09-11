use super::charset::{create_base_charset, swap_chars, swap_ranges, build_lookup_table};

#[derive(Copy, Clone)]
pub struct Asc100Version {
    pub name: &'static str,
    pub charset: [char; 100],
    pub lookup: [u8; 128],
}

const fn create_v1_standard() -> [char; 100] {
    let base = create_base_charset();
    // V1: Swap space (index 0) with tilde (index 94)
    swap_chars(base, 0, 94)
}

const fn create_v2_numbers_first() -> [char; 100] {
    let base = create_base_charset();
    // V2: Move numbers to the front for optimization
    // Numbers are at indices 16-25 (0-9), move to 0-9
    swap_ranges(base, 0, 10, 16, 10)
}

const fn create_v3_lowercase_first() -> [char; 100] {
    let base = create_base_charset();
    // V3: Lowercase letters first (most common in text)
    // Lowercase at indices 65-90, move to front
    swap_ranges(base, 0, 26, 65, 26)
}

const fn create_v4_url_optimized() -> [char; 100] {
    let mut base = create_base_charset();
    // V4: Optimize for URL-like content
    // Put common URL chars early: lowercase, numbers, dash, dot, slash
    
    // First swap lowercase to front
    base = swap_ranges(base, 0, 26, 65, 26);
    // Then move numbers after lowercase
    base = swap_ranges(base, 26, 10, 42, 10);
    base
}

pub const V1_STANDARD: Asc100Version = Asc100Version {
    name: "v1_standard",
    charset: create_v1_standard(),
    lookup: build_lookup_table(create_v1_standard()),
};

pub const V2_NUMBERS: Asc100Version = Asc100Version {
    name: "v2_numbers_first",
    charset: create_v2_numbers_first(),
    lookup: build_lookup_table(create_v2_numbers_first()),
};

pub const V3_LOWERCASE: Asc100Version = Asc100Version {
    name: "v3_lowercase_first",
    charset: create_v3_lowercase_first(),
    lookup: build_lookup_table(create_v3_lowercase_first()),
};

pub const V4_URL: Asc100Version = Asc100Version {
    name: "v4_url_optimized",
    charset: create_v4_url_optimized(),
    lookup: build_lookup_table(create_v4_url_optimized()),
};

impl Asc100Version {
    pub fn encode(&self, input: &str) -> Result<String, crate::Asc100Error> {
        crate::encode(input, &self.charset, &self.lookup)
    }
    
    pub fn decode(&self, encoded: &str) -> Result<String, crate::Asc100Error> {
        crate::decode(encoded, &self.charset)
    }
    
    pub fn display_charset(&self) {
        println!("Version: {}", self.name);
        println!("Charset mapping (first 20):");
        for i in 0..20 {
            let ch = self.charset[i];
            let display = match ch {
                '\0' => "\\0".to_string(),
                '\t' => "\\t".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\x01' => "\\x01".to_string(),
                c => c.to_string(),
            };
            println!("  [{}]: '{}'", i, display);
        }
        println!("  ...");
    }
}