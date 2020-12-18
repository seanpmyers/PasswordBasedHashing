use crate::ihash_encryption_service::HashCheckResult;
use crate::ihash_encryption_service::HashResult;
use crate::ihash_encryption_service::IHashEncryptionService;
use base64;
use ring::rand::SecureRandom;
use ring::{pbkdf2, rand};
use std::num::NonZeroU32;

pub struct HashEncryptionService();


impl IHashEncryptionService for HashEncryptionService {
   const SALT_SIZE: usize = 16; 
   const KEY_SIZE: usize = 32;
   
   fn new() -> HashEncryptionService {
      return HashEncryptionService{};
   }

   fn hash(input: &str) -> HashResult {
      let n_iter = NonZeroU32::new(10000).unwrap();
      let rng = rand::SystemRandom::new();
      
      let mut salt = [0u8; Self::SALT_SIZE];

      rng.fill(&mut salt)?;
      println!("Salt: {}", base64::encode(&salt));


      let mut pbkdf2_hash = [0u8; Self::KEY_SIZE];
      pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        n_iter,
        &salt,
        input.as_bytes(),
        &mut pbkdf2_hash,
      );
      let hashed_input = format!("{}.{}.{}",10000,base64::encode(&salt),base64::encode(&pbkdf2_hash));//{iterations.salt.key}
      println!("PBKDF2 hash: {}", hashed_input);

      
      return Ok(hashed_input);
   }
   
   fn check(hash: &str, input: &str) -> HashCheckResult{
      let is_equivalent: bool = false;
      let needs_upgrade: bool = false;

      let mut parts = hash.split(".");
      
      // hash()

      return Ok((is_equivalent, needs_upgrade));
      // return Err("Something went wrong comparing inputs!");
   }
}