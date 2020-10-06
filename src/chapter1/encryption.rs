/// Corresponds to CCSP in Python, Section 1.3, titled "Unbreakable Encryption"
///
/// Original approach mirrored the elixir version, but ran into several troubles decrypting...
/// ended up following the CCSPiJ style of byte arrays, which come across cleaner to read anyways.
///
/// https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter1/UnbreakableEncryption.java
use rand::{thread_rng, Rng};
use std::str;

fn random_key(length: usize) -> Vec<u8> {
    // there might be a more standard way to fill a vector with random data
    let mut buf = vec![0 as u8; length];
    thread_rng().fill(&mut buf[..]);
    buf
}

fn encrypt(original: &str) -> (Vec<u8>, Vec<u8>) {
    let original_bytes: &[u8] = original.as_bytes();
    let dummy_key: Vec<u8> = random_key(original.len());
    let mut encrypted_key = vec![0 as u8; original.len()];

    for i in 0..original_bytes.len() {
        encrypted_key[i] = original_bytes[i] ^ dummy_key[i];
    }

    (dummy_key, encrypted_key)
}

fn decrypt(keys: (Vec<u8>, Vec<u8>)) -> String {
    let (key1, key2) = keys;
    let mut decrypted: Vec<u8> = vec![0 as u8; key1.len()];
    for i in 0..key1.len() {
        decrypted[i] = key1[i] ^ key2[i];
    }

    // this line is not a best practice
    String::from(str::from_utf8(&decrypted).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let original = "the cat";
        let res = encrypt(original);
        let message = decrypt(res);
        assert_eq!(message, original);

        let original = "";
        let res = encrypt(original);
        let message = decrypt(res);
        assert_eq!(message, original);

        let original = "aaaaaaaaaaaa";
        let res = encrypt(original);
        let message = decrypt(res);
        assert_eq!(message, original);

        let original = "Soluta recusandae sint voluptas commodi nulla. Perspiciatis rerum odit dolore dolor voluptate et. Quia est ex neque dolores.";
        let res = encrypt(original);
        let message = decrypt(res);
        assert_eq!(message, original);
    }
}
