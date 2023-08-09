const UPPERCASE_ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const LOWERCASE_ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// Checks if a string contains only ASCII alphabetic characters.
pub fn is_clean(text: &str) -> bool {
    !text.chars().any(|c| !c.is_ascii_alphabetic())
}

/// Generates a repeated key to match the length of the input text.
pub fn repeat_key(text: &str, key: &str) -> String {
    key.chars().cycle().take(text.len()).collect()
}

/// Adds a character to the alphabet after shifting it by an amount determined by the key.
/// The key must contain only alphabetic characters.
pub fn add_char(c: char, key: char) -> char {
    let mut k_pos = UPPERCASE_ALPHABET
        .iter()
        .position(|&k| k == key.to_ascii_uppercase())
        .expect("Key contains non alphabetic characters");

    // The key is 0-indexed but the alphabet is 1-indexed
    k_pos += 1;

    shift_char(c, |pos| pos + k_pos)
}

/// Shifts a character across the alphabet using a custom index function.
pub fn shift_char<F>(c: char, get_index: F) -> char
where
    F: Fn(usize) -> usize,
{
    let c_pos = get_position(c, &UPPERCASE_ALPHABET)
        .or_else(|| get_position(c, &LOWERCASE_ALPHABET))
        .or_else(|| get_position(c, &NUMBERS));

    match c_pos {
        Some(c_pos) => {
            let new_position = get_index(c_pos) as i32;

            if c.is_numeric() {
                NUMBERS[modulo(new_position, 10) as usize]
            } else if c.is_uppercase() {
                UPPERCASE_ALPHABET[modulo(new_position, 26) as usize]
            } else {
                LOWERCASE_ALPHABET[modulo(new_position, 26) as usize]
            }
        }
        None => c,
    }
}

/// Returns the position of a character in a specific alphabet.
fn get_position(c: char, alphabet: &[char]) -> Option<usize> {
    alphabet.iter().position(|&x| x == c)
}

/// Calculates the modulo of a number, including support for negative numbers.
fn modulo(n: i32, m: i32) -> i32 {
    ((n % m) + m) % m
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_for_special_characters() {
        assert!(is_clean("AGoodKey"));
        assert_eq!(is_clean("P3ssima Ch4av3!! lul"), false);
    }

    #[test]
    fn test_repeat_key() {
        assert_eq!(
            repeat_key("A median sized text", "Key"),
            "KeyKeyKeyKeyKeyKeyK"
        );
        assert_eq!(
            repeat_key("Text", "KeyBiggerThanTheText"),
            "KeyBiggerThanTheText"
        );
    }

    #[test]
    fn test_add_char() {
        assert_eq!(add_char('a', 'b'), 'c');
        assert_eq!(add_char('a', 'z'), 'a');
        assert_eq!(add_char('z', 'a'), 'a');
        assert_eq!(add_char('a', 'Z'), 'a');
        assert_eq!(add_char('1', 'a'), '2');
        assert_eq!(add_char(' ', 'g'), ' ');
        assert_eq!(add_char('A', 'A'), 'B');
        assert_eq!(add_char('T', 'c'), 'W');
    }

    #[test]
    fn test_modulo() {
        assert_eq!(modulo(5, 3), 2);
        assert_eq!(modulo(-5, 3), 1);
        assert_eq!(modulo(5, -3), -1);
        assert_eq!(modulo(-5, -3), -2);
    }

    #[test]
    fn test_shift_char() {
        assert_eq!(shift_char('a', |x| x + 1), 'b');
        assert_eq!(shift_char('a', |x| x + 2), 'c');

        assert_eq!(shift_char('z', |x| x + 1), 'a');
        assert_eq!(shift_char('z', |x| x + 2), 'b');

        assert_eq!(shift_char('A', |x| x + 1), 'B');

        assert_eq!(shift_char('Z', |x| x + 1), 'A');
    }
}
