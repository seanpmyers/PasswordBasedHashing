use crate::ihash_encryption_service::IHashEncryptionService;

pub struct HashEncryptionService;

impl IHashEncryptionService for HashEncryptionService {
   const SALT_SIZE: i32 = 16; 
   const KEY_SIZE: i32 = 32;
   fn new() -> HashEncryptionService {
      return HashEncryptionService{};
   }
   fn hash(input: &str) -> &str {
      return input;
   }
   fn check(hash: &str, input: &str) -> (bool, bool){
      return (true, false);
   }
}