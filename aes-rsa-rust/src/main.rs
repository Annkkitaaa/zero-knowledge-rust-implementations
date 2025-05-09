mod aes_encryption;
mod rsa_encryption;

fn main() {
    println!("--- AES Encryption Demo ---");
    aes_encryption::aes_demo();

    println!("\n--- RSA Encryption Demo ---");
    rsa_encryption::rsa_demo();
}
