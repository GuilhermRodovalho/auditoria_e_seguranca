use crate::utils::alphabet::{add_char, is_clean, repeat_key, subtract_char};
use crate::utils::cipher::Cipher;

struct Vigenere {
    key: String,
}

impl Cipher for Vigenere {
    type KeyType = String;

    fn new(key: Self::KeyType) -> Self {
        if !is_clean(&key) {
            panic!("The key must not contains special characters and must be all alphabetic")
        }

        Vigenere { key }
    }

    fn encrypt(&self, text: &str) -> String {
        let repeated_key = repeat_key(&text, &self.key);
        let mut cipher_text = String::new();
        for (c, k) in text.chars().zip(repeated_key.chars()) {
            cipher_text.push(add_char(c, k));
        }
        cipher_text
    }

    fn decrypt(&self, text: &str) -> String {
        let repeated_key = repeat_key(&text, &self.key);
        let mut plain_text = String::new();
        for (c, k) in text.chars().zip(repeated_key.chars()) {
            plain_text.push(subtract_char(c, k));
        }
        plain_text
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vigenere_cipher_encrypt() {
        assert_eq!(
            "Wzn wjoe ex hkw cidqmtyfx".to_string(),
            Vigenere::new("cripticsspren".to_string()).encrypt("THECALLOFTHEMOUNTAINS")
        );
        // generate other tests
    }
}
