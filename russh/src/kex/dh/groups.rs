use hex_literal::hex;
use num_bigint::{BigUint, RandBigInt};
use rand;

pub struct DhGroup {
    pub(crate) prime: &'static [u8],
    pub(crate) generator: usize,
    pub(crate) exp_size: u64,
}

pub const DH_GROUP1: DhGroup = DhGroup {
    prime: hex!(
        "
        FFFFFFFF FFFFFFFF C90FDAA2 2168C234 C4C6628B 80DC1CD1
         29024E08 8A67CC74 020BBEA6 3B139B22 514A0879 8E3404DD
         EF9519B3 CD3A431B 302B0A6D F25F1437 4FE1356D 6D51C245
         E485B576 625E7EC6 F44C42E9 A637ED6B 0BFF5CB6 F406B7ED
         EE386BFB 5A899FA5 AE9F2411 7C4B1FE6 49286651 ECE65381
         FFFFFFFF FFFFFFFF
        "
    )
    .as_slice(),
    generator: 2,
    exp_size: 256,
};

pub const DH_GROUP14: DhGroup = DhGroup {
    prime: hex!(
        "
        FFFFFFFF FFFFFFFF C90FDAA2 2168C234 C4C6628B 80DC1CD1
        29024E08 8A67CC74 020BBEA6 3B139B22 514A0879 8E3404DD
        EF9519B3 CD3A431B 302B0A6D F25F1437 4FE1356D 6D51C245
        E485B576 625E7EC6 F44C42E9 A637ED6B 0BFF5CB6 F406B7ED
        EE386BFB 5A899FA5 AE9F2411 7C4B1FE6 49286651 ECE45B3D
        C2007CB8 A163BF05 98DA4836 1C55D39A 69163FA8 FD24CF5F
        83655D23 DCA3AD96 1C62F356 208552BB 9ED52907 7096966D
        670C354E 4ABC9804 F1746C08 CA18217C 32905E46 2E36CE3B
        E39E772C 180E8603 9B2783A2 EC07A28F B5C55DF0 6F4C52C9
        DE2BCBF6 95581718 3995497C EA956AE5 15D22618 98FA0510
        15728E5A 8AACAA68 FFFFFFFF FFFFFFFF
        "
    )
    .as_slice(),
    generator: 2,
    exp_size: 256,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DH {
    prime_num: BigUint,
    generator: usize,
    exp_size: u64,
    private_key: BigUint,
    public_key: BigUint,
    shared_secret: BigUint,
}

impl DH {
    pub fn new(group: &DhGroup) -> Self {
        Self {
            prime_num: BigUint::from_bytes_be(group.prime),
            generator: group.generator,
            exp_size: group.exp_size,
            private_key: BigUint::default(),
            public_key: BigUint::default(),
            shared_secret: BigUint::default(),
        }
    }

    pub fn generate_private_key(&mut self, is_server: bool) -> BigUint {
        let q = (&self.prime_num - &BigUint::from(1u8)) / &BigUint::from(2u8);
        let mut rng = rand::thread_rng();
        self.private_key =
            rng.gen_biguint_range(&if is_server { 1u8.into() } else { 2u8.into() }, &q);
        self.private_key.clone()
    }

    pub fn generate_public_key(&mut self) -> BigUint {
        self.public_key = BigUint::from(self.generator).modpow(&self.private_key, &self.prime_num);
        self.public_key.clone()
    }

    pub fn compute_shared_secret(&mut self, other_public_key: BigUint) -> BigUint {
        self.shared_secret = other_public_key.modpow(&self.private_key, &self.prime_num);
        self.shared_secret.clone()
    }

    pub fn validate_shared_secret(&self, shared_secret: &BigUint) -> bool {
        let one = BigUint::from(1u8);
        let prime_minus_one = &self.prime_num - &one;

        shared_secret > &one && shared_secret < &prime_minus_one
    }

    pub fn decode_public_key(buffer: &[u8]) -> BigUint {
        BigUint::from_bytes_be(buffer)
    }

    pub fn validate_public_key(&self, public_key: &BigUint) -> bool {
        let one = BigUint::from(1u8);
        let prime_minus_one = &self.prime_num - &one;

        public_key > &one && public_key < &prime_minus_one
    }
}
