use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use base64::{encode, decode};

pub fn rsa_demo() {
    let mut rng = OsRng;
    let bits = 2048;

    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate key");
    let public_key = RsaPublicKey::from(&private_key);

    let message = b"Hi Ankita!";
    let padding = PaddingScheme::new_pkcs1v15_encrypt();

    // Encrypt
    let enc_data = public_key.encrypt(&mut rng, padding, &message[..]).expect("encryption failed");
    println!("[RSA] Encrypted: {}", encode(&enc_data));

    // Decrypt
    let dec_data = private_key.decrypt(padding, &enc_data).expect("decryption failed");
    println!("[RSA] Decrypted: {}", String::from_utf8(dec_data).unwrap());
}
