use crate::ihash_encryption_service::HashCheckResult;
use crate::ihash_encryption_service::HashResult;
use crate::ihash_encryption_service::IHashEncryptionService;
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

pub struct HashEncryptionService();


impl IHashEncryptionService for HashEncryptionService {
   const SALT_SIZE: u8 = 16; 
   const KEY_SIZE: u8 = 32;
   
   fn new() -> HashEncryptionService {
      return HashEncryptionService{};
   }

   fn hash(input: &str) -> HashResult {
      const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
      let n_iter = NonZeroU32::new(100_000).unwrap();
      let rng = rand::SystemRandom::new();
      
      let mut salt = [0u8; CREDENTIAL_LEN];
      println!("Salt: {}", HEXUPPER.encode(&salt));

      rng.fill(&mut salt)?;

      let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
      pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        input.as_bytes(),
        &mut pbkdf2_hash,
      );
      let hashed_input = HEXUPPER.encode(&pbkdf2_hash);
      println!("PBKDF2 hash: {}", hashed_input);

      
      return Ok(hashed_input);
   }
   
   fn check(hash: &str, input: &str) -> HashCheckResult{
      let is_equivalent: bool = false;
      let needs_upgrade: bool = false;

      // hash()

      return Ok((is_equivalent, needs_upgrade));
      // return Err("Something went wrong comparing inputs!");
   }
}