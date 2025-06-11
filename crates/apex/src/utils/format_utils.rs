use num_format::{Locale, ToFormattedString};

pub fn format_u64(n: u64) -> String {
    n.to_formatted_string(&Locale::en)        // 123 456 789 → "123,456,789"
}

pub fn format_u32(n: u32) -> String {
    n.to_formatted_string(&Locale::en)
}
