pub trait Cipher {
    type KeyType;

    fn new(key: Self::KeyType) -> Self;
    fn encrypt(&self, text: &str) -> String;
    fn decrypt(&self, text: &str) -> String;
}
