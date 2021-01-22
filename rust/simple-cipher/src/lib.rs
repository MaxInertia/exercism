pub fn encode(key: &str, s: &str) -> Option<String> {
    return code(key, s, Variant::Encode);
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    return code(key, s, Variant::Decode);
}

enum Variant {
    Encode,
    Decode,
}

fn code(key: &str, s: &str, variant: Variant) -> Option<String> {
    let shift_direction: i16 = match variant {
        Variant::Encode => 1,
        Variant::Decode => -1,
    };

    if key.len() == 0 {
        return None;
    }

    for key_char in key.chars() {
        let value = key_char as u8;
        if value > 122 || value < 97 {
            return None;
        }
    }

    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        let new_char = apply_shift_with_key(c, key, i, shift_direction);
        result += &new_char.to_string();
    }

    return Some(result);
}

fn apply_shift_with_key(c: char, key: &str, i: usize, shift_direction: i16) -> char {
    let key_char: char = match key.chars().nth(i) {
        Some(a_char) => a_char,
        None => 0 as char,
    };

    let c_digit = c as i16;
    let shift = key_char as i16 - 97;

    let mut new_digit = c_digit + shift * shift_direction;
    new_digit += if new_digit > 122 {
        -26
    } else if new_digit < 97 {
        26
    } else {
        0
    };

    return new_digit as u8 as char;
}

use rand::Rng;

// Generates a random key with only a-z chars and encode {}. Return tuple (key, encoded s)
pub fn encode_random(s: &str) -> (String, String) {
    let key_size = if s.len() >= 100 { s.len() } else { 100 };

    let mut key = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..key_size {
        let new_key_char = rng.gen_range(97 as u8, 122 as u8) as char;
        key += &new_key_char.to_string()
    }

    return match encode(key.as_str(), s) {
        Some(encoded_s) => (key, encoded_s),
        None => (key, String::new()),
    };
}
