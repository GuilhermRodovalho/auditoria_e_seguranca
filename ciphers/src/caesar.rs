use crate::utils::{alphabet::shift_char, cipher::Cipher};

struct Caesar {
    key: u8,
}

impl Cipher for Caesar {
    type KeyType = u8;

    fn new(key: u8) -> Caesar {
        if key == 0 || key >= 26 {
            panic!("Key must be in the interval 1 <= key < 26");
        }
        Caesar { key }
    }

    fn encrypt(&self, text: &str) -> String {
        let mut result = String::new();
        for c in text.chars() {
            result.push(shift_char(c, |pos| pos + (self.key as usize)));
        }
        result
    }

    fn decrypt(&self, text: &str) -> String {
        let mut result = String::new();
        for c in text.chars() {
            result.push(shift_char(c, |pos| pos - (self.key as usize)));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_caesar_cipher_encrypt() {
        let cipher = Caesar::new(2);
        let encripted = cipher.encrypt("The brown fox jumped over the lazy dog");
        assert_eq!(
            encripted,
            "Vjg dtqyp hqz lworgf qxgt vjg ncba fqi".to_string()
        );
    }

    #[test]
    fn test_caesar_cipher_decrypt() {
        let cipher = Caesar::new(2);
        let decripted = cipher.decrypt("Fcpk Ecnkhqtpkc");
        assert_eq!(decripted, "Dani California".to_string());
    }
}
