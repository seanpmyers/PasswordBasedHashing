use ring::error;

pub trait IHashEncryptionService{  
   const SALT_SIZE: u8; 
   const KEY_SIZE: u8;
   fn new() -> Self;
   fn hash(input: &str) -> Result<&str, error::Unspecified>;
   fn check(hash: &str, input: &str) -> (bool, bool);
}