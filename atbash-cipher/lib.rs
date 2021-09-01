pub mod atbash_cipher;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    atbash_cipher::encode(plain)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    atbash_cipher::decode(cipher)
}
