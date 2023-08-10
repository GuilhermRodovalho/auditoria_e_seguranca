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

// Trait to get the position of a character in the alphabet.
trait PositionInAlphabet {
    fn position_in_alphabet(&self) -> Option<usize>;
}

impl PositionInAlphabet for char {
    fn position_in_alphabet(&self) -> Option<usize> {
        get_position(*self, &UPPERCASE_ALPHABET)
            .or_else(|| get_position(*self, &LOWERCASE_ALPHABET))
            .or_else(|| get_position(*self, &NUMBERS))
    }
}

fn get_new_char(c: char, position: i32) -> char {
    if c.is_numeric() {
        NUMBERS[modulo(position, 10) as usize]
    } else if c.is_uppercase() {
        UPPERCASE_ALPHABET[modulo(position, 26) as usize]
    } else {
        LOWERCASE_ALPHABET[modulo(position, 26) as usize]
    }
}

pub fn shift_char_with_key<F>(c: char, k: char, func: F) -> char
where
    F: Fn(usize, usize) -> usize,
{
    let c_pos = c.position_in_alphabet();

    let k_pos = k.position_in_alphabet().expect("Key must be alphabetic");

    match c_pos {
        Some(c_pos) => {
            let new_position = func(c_pos, k_pos) as i32;

            get_new_char(c, new_position)
        }
        None => c,
    }
}

/// Shifts a character across the alphabet using a custom index function.
pub fn shift_char<F>(c: char, get_index: F) -> char
where
    F: Fn(usize) -> usize,
{
    let c_pos = c.position_in_alphabet();

    match c_pos {
        Some(c_pos) => {
            let new_position = get_index(c_pos) as i32;

            get_new_char(c, new_position)
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
        assert_eq!(repeat_key("Text", "KeyBiggerThanTheText"), "KeyB");
    }

    #[test]
    fn test_add_char() {
        // adding 1 because the alphabet is 1-indexed and the arrays are 0-indexed
        let get_index = |c_pos, k_pos| c_pos + k_pos + 1;

        assert_eq!(shift_char_with_key('a', 'b', get_index), 'c');
        assert_eq!(shift_char_with_key('a', 'z', get_index), 'a');
        assert_eq!(shift_char_with_key('z', 'a', get_index), 'a');
        assert_eq!(shift_char_with_key('a', 'Z', get_index), 'a');
        assert_eq!(shift_char_with_key('1', 'a', get_index), '2');
        assert_eq!(shift_char_with_key(' ', 'g', get_index), ' ');
        assert_eq!(shift_char_with_key('A', 'A', get_index), 'B');
        assert_eq!(shift_char_with_key('T', 'c', get_index), 'W');
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
