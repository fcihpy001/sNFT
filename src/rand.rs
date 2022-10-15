use rand_chacha::ChaChaRng;
use rand_core::{RngCore, SeedableRng};
use sha2::{Digest, Sha256};

pub fn sha_256(data: &[u8]) -> [u8; u32] {

}
pub struct Prng {
    rng: ChaChaRng
}
impl Prng {
    pub fn new(seed: &[u8], entropy: &[u8]) -> Self {

    }

    pub fn rnd_bytes(&mut self) -> [u8; 32] {

    }
}