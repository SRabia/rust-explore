extern crate crypto;

use std::vec;

use crypto::aead::AeadEncryptor;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::aes_gcm;

pub fn cmd_sha256<E>(input: E)-> Vec<u8>
where
E: AsRef<[u8]>,
{
    let mut sha = Sha256::new();
    let input = input.as_ref();
    sha.input(input);
    let out:&mut [u8] = &mut [0;32];
    sha.result(out);
    out.to_vec()

}

#[derive(Debug)]
pub struct TaggedEncryption<T, E> {
    pub tag: T,
    pub out: E,
}

impl<T, E> TaggedEncryption<T, E> {
    pub fn build(tag: T, out:E) ->Self {
        Self{tag, out}
    }
}


pub fn cmd_aes128_gcm_tag<E>(input: E, key: E, iv: E, additional: E) -> TaggedEncryption<Vec<u8>, Vec<u8>>
where
E: AsRef<[u8]>,
{
    let key_size = crypto::aes::KeySize::KeySize128;
    let mut key16 = [0;16];
    let mut nonce12 = [0;12];
    for i in 0..16{
        key16[i] = *(key.as_ref()
            .get(i)
            .unwrap_or(&0));
    }
    for i in 0..12{
        nonce12[i] = *(iv.as_ref()
            .get(i)
            .unwrap_or(&0));
    }

    let aad = additional.as_ref();

    let mut encrypter = aes_gcm::AesGcm::new(key_size, key16.as_ref(), nonce12.as_ref(), aad);
    let tag64: &mut [u8]= &mut [0;16];

    let input_bytes = input.as_ref();
    let mut output: Vec<u8> = vec![0; input_bytes.len()];
    encrypter.encrypt(input.as_ref(), &mut output, tag64);

    TaggedEncryption::build(tag64.to_vec(), output)

}

