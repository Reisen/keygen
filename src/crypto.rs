//! General helpers for working with Crypto primitives.

use hmac::{Hmac, Mac};
use sha2::Sha256;

pub struct Entropy {
    hmac:     Hmac<Sha256>,
    consumed: usize,
}

impl Entropy {
    pub fn new<T, U>(secret: T, salt: U) -> Self
    where
        T: AsRef<[u8]>,
        U: AsRef<[u8]>,
    {
        let mut hmac = Hmac::<Sha256>::new(secret.as_ref().into());
        hmac.input(salt.as_ref());
        Self { hmac, consumed: 0 }
    }

    /// Get N bytes for some named secret.
    pub fn get_bytes(&mut self, length: usize) -> Vec<u8> {
        let mut blocks = (length / 32) + std::cmp::min(1, length % 32);
        let mut output = Vec::new();
        while blocks > 0 {
            let bytes = self.hmac.clone().result().code();
            self.hmac.input(&bytes);
            output.extend(&bytes);
            blocks -= 1;
        }

        self.consumed += output.len();
        output.truncate(length);
        assert_eq!(output.len(), length);
        output
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_expected_bytes() {
        let pass = &[0u8; 64];
        let salt = b"Hello World";

        let mut secret_generator = Entropy::new(&pass[.. 64], salt);

        let output = secret_generator.get_bytes(31);
        assert_eq!(output.len(), 31);
        let output = secret_generator.get_bytes(32);
        assert_eq!(output.len(), 32);
        let output = secret_generator.get_bytes(64);
        assert_eq!(output.len(), 64);
        let output = secret_generator.get_bytes(65);
        assert_eq!(output.len(), 65);

        assert_eq!(secret_generator.consumed, 224);
    }
}
