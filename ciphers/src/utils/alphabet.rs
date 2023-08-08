const UPPERCASE_ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const LOWERCASE_ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn is_clean(text: &str) -> bool {
    !text.chars().any(|c| !c.is_ascii_alphabetic())
}

pub fn repeat_key(text: &str, key: &str) -> String {
    if key.len() > text.len() {
        return key.to_string();
    }
    key.chars().cycle().take(text.len()).collect()
}

pub fn add_char(c: char, key: char) -> char {
    let c_pos = UPPERCASE_ALPHABET.iter().position(|&x| x == c);
    let k_pos = UPPERCASE_ALPHABET
        .iter()
        .position(|&k| k == key.to_ascii_uppercase())
        .expect("Key contains non alphabetic characters");

    if let Some(c_pos) = c_pos {
        return *UPPERCASE_ALPHABET.get((k_pos + c_pos + 1) % 26).unwrap();
    }
    let c_pos = LOWERCASE_ALPHABET.iter().position(|&x| x == c);

    if let Some(c_pos) = c_pos {
        return *LOWERCASE_ALPHABET.get((k_pos + c_pos + 1) % 26).unwrap();
    }

    let c_pos = NUMBERS.iter().position(|&x| x == c);

    match c_pos {
        Some(c_pos) => *NUMBERS.get((k_pos + c_pos + 1) % 26).unwrap(),
        None => c,
    }
}

pub fn add_char_with_shift(c: char, shift: usize) -> char {
    let mut c_pos = NUMBERS.iter().position(|&x| x == c);

    if c_pos.is_none() {
        c_pos = UPPERCASE_ALPHABET
            .iter()
            .position(|&x| x == c.to_uppercase().next().unwrap());
    }

    match c_pos {
        Some(c_pos) => {
            let new_position = c_pos + shift;

            if c.is_numeric() {
                NUMBERS[new_position % 10]
            } else if c.is_uppercase() {
                UPPERCASE_ALPHABET[new_position % 26]
            } else {
                LOWERCASE_ALPHABET[new_position % 26]
            }
        }
        None => c,
    }
}

pub fn subtract_char_with_shift<F>(c: char, get_index: F) -> char
where
    F: Fn(usize) -> usize,
{
    let mut c_pos = NUMBERS.iter().position(|&x| x == c);

    if c_pos.is_none() {
        c_pos = UPPERCASE_ALPHABET
            .iter()
            .position(|&x| x == c.to_uppercase().next().unwrap());
    }

    match c_pos {
        Some(c_pos) => {
            let new_position = get_index(c_pos);

            if c.is_numeric() {
                NUMBERS[modulo(new_position as i32, 10 as i32) as usize]
            } else if c.is_uppercase() {
                UPPERCASE_ALPHABET[modulo(new_position as i32, 26 as i32) as usize]
            } else {
                LOWERCASE_ALPHABET[modulo(new_position as i32, 26 as i32) as usize]
            }
        }
        None => c,
    }
}
// {
//     let mut c_pos = NUMBERS.iter().position(|&x| x == c);

//     if c_pos.is_none() {
//         c_pos = UPPERCASE_ALPHABET
//             .iter()
//             .position(|&x| x == c.to_uppercase().next().unwrap());
//     }

//     match c_pos {
//         Some(c_pos) => {
//             let new_position = c_pos - shift;

//             if c.is_numeric() {
//                 NUMBERS[modulo(new_position as i32, 10 as i32) as usize]
//             } else if c.is_uppercase() {
//                 UPPERCASE_ALPHABET[modulo(new_position as i32, 26 as i32) as usize]
//             } else {
//                 LOWERCASE_ALPHABET[modulo(new_position as i32, 26 as i32) as usize]
//             }
//         }
//         None => c,
//     }
// }

// Function that calculates the modulo of a number
// works with negative numbers
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
    }
}
