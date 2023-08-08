use crate::utils::{is_clean, repeat_key, Cipher};

struct Vigenere {
    key: String,
}

impl Cipher for Vigenere {
    type KeyType = String;

    fn new(key: Self::KeyType) -> Self {
        if !is_clean(&key) {
            panic!("The key should not contains special characters and must be all alphabetic")
        }

        Vigenere { key }
    }

    fn encrypt(&self, text: &str) -> String {
        let repeated_key = repeat_key(&text, &self.key);
        let mut cipher_text = String::new();
        for c in text.chars() {
            cipher_text.push()
        }
    }

    fn decrypt(&self, text: &str) -> String {
        todo!()
    }
}
