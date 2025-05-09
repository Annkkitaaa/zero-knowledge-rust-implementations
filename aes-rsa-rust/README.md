
# 🔐 AES & RSA Encryption in Rust

This project demonstrates implementations of **AES (Advanced Encryption Standard)** and **RSA (Rivest–Shamir–Adleman)** encryption algorithms in Rust using safe, modern cryptographic crates.


---

## 📂 Project Structure

```

aes-rsa-rust/
├── src/
│   ├── main.rs              # Entry point
│   ├── aes\_encryption.rs    # AES demo implementation
│   └── rsa\_encryption.rs    # RSA demo implementation
├── Cargo.toml               # Project metadata and dependencies
└── README.md                # Project documentation

````

---

##  Content

- How AES-128 works in CBC mode with padding
- How RSA works with key generation, encryption, and decryption
- How to use Rust crates for cryptography: `aes`, `rsa`, `block-modes`, etc.
- Basic structure for modular Rust projects

---

## Setup & Running

### 1️⃣ Clone the repository

```bash
git clone https://github.com/yourusername/aes-rsa-rust.git
cd aes-rsa-rust
````

### 2️⃣ Install dependencies

Ensure you have Rust installed:
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Then run:

```bash
cargo build
```

### 3️⃣ Run the project

```bash
cargo run
```

---

## 🔐 AES Encryption (Symmetric)

* **Mode**: AES-128 in CBC mode with PKCS#7 padding
* **Key size**: 128 bits (16 bytes)
* **IV**: 128 bits (16 bytes)

### 📄 `aes_encryption.rs` highlights:

```rust
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

let key = b"very secret key."; // 16 bytes
let iv = b"unique nonce1234"; // 16 bytes

let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
let ciphertext = cipher.encrypt_vec(plaintext);
```

---

## 🔐 RSA Encryption (Asymmetric)

* **Key size**: 2048 bits
* **Padding**: PKCS1v15
* **Randomness**: Uses `rand::rngs::OsRng`

### 📄 `rsa_encryption.rs` highlights:

```rust
let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
let public_key = RsaPublicKey::from(&private_key);

let ciphertext = public_key.encrypt(&mut rng, padding, &message).unwrap();
let decrypted = private_key.decrypt(padding, &ciphertext).unwrap();
```

---

## 📦 Dependencies

```toml
[dependencies]
aes = "0.8"
block-modes = "0.9"
block-padding = "0.3"
hex = "0.4"
rsa = "0.9"
rand = "0.8"
base64 = "0.21"
```

---

## 📚 Background Concepts

### AES

* Block cipher standardized by NIST.
* Operates on 128-bit blocks with keys of 128/192/256 bits.
* Involves SubBytes, ShiftRows, MixColumns, AddRoundKey steps.
* Uses Galois field math (GF(2^8)) for operations.

### RSA

* Based on hardness of factoring large primes.
* Key generation uses Euler’s totient function.
* Encryption: $c = m^e \mod n$
* Decryption: $m = c^d \mod n$


## 🧑‍💻 Author

Ankita Singh
GitHub: [@Annkkitaaa](https://github.com/Annkkitaaa)

---
