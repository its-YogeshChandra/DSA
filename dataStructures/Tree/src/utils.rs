use std::sync::atomic::{AtomicU64, Ordering};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

const NAME_LENGTH: usize = 15;
const HASH_SEED: &[u8] = b"phonebook-seed";
static NEXT_NUMBER: AtomicU64 = AtomicU64::new(1);

/// Returns a random-looking name and its unique sequence number.
///
/// The sequence is unique within this process. Hashing it with SHA-256 makes
/// the name look random, and URL-safe Base64 converts the hash to text.
pub fn generate_random_name_and_number() -> (String, u64) {
    let number = NEXT_NUMBER.fetch_add(1, Ordering::Relaxed);

    let mut hasher = Sha256::new();
    hasher.update(HASH_SEED);
    hasher.update(number.to_be_bytes());

    let encoded_hash = URL_SAFE_NO_PAD.encode(hasher.finalize());
    let name = format!("N{}", &encoded_hash[..NAME_LENGTH]);

    (name, number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_distinct_names_and_sequential_numbers() {
        let (first_name, first_number) = generate_random_name_and_number();
        let (second_name, second_number) = generate_random_name_and_number();

        assert_eq!(second_number, first_number + 1);
        assert_ne!(first_name, second_name);
        assert_eq!(first_name.len(), NAME_LENGTH + 1);
        assert!(first_name.starts_with('N'));
    }
}
