use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub fn aes_demo() {
    let key = b"very secret key.";  // 16 bytes
    let iv = b"unique nonce1234";  // 16 bytes
    let plaintext = b"Secret message for ankita!";

    // Encrypt
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    let ciphertext = cipher.encrypt_vec(plaintext);
    println!("[AES] Ciphertext (hex): {}", hex::encode(&ciphertext));

    // Decrypt
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    let decrypted = cipher.decrypt_vec(&ciphertext).unwrap();
    println!("[AES] Decrypted text: {}", String::from_utf8(decrypted).unwrap());
}
