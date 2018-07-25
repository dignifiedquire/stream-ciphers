#![no_std]
extern crate aes_ctr;

use aes_ctr::{Aes128Ctr, Aes256Ctr};
use aes_ctr::stream_cipher::generic_array::GenericArray;
use aes_ctr::stream_cipher::{StreamCipherCore, NewFixStreamCipher};

macro_rules! impl_test {
    ($name:ident, $cipher:ty, $data:expr) => {
        #[test]
        fn $name() {
            let key = include_bytes!(concat!("data/", $data, ".key.bin"));
            let key = GenericArray::from_slice(key);
            let iv = include_bytes!(concat!("data/", $data, ".iv.bin"));
            let iv = GenericArray::from_slice(iv);
            let plaintext = include_bytes!(
                concat!("data/", $data, ".plaintext.bin"));
            let ciphertext = include_bytes!(
                concat!("data/", $data, ".ciphertext.bin"));

            for i in 1..256 {
                let mut mode = <$cipher>::new(key, iv);
                let mut pt = plaintext.to_vec();
                for chunk in pt.chunks_mut(i) {
                    mode.apply_keystream(chunk);
                }
                assert_eq!(pt, &ciphertext[..]);
            }
        }
    }
}

// Random tests generated by OpenSSL
impl_test!(aes128_ctr, Aes128Ctr, "ctr_aes128");
impl_test!(aes256_ctr, Aes256Ctr, "ctr_aes256");