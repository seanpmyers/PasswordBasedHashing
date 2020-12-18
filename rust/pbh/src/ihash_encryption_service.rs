use std::fmt;

pub enum HashError {
   CryptoError,

   Error,
   // [...]
}

impl From<ring::error::Unspecified> for HashError {
   fn from(_: ring::error::Unspecified) -> Self { HashError::CryptoError }
}

impl fmt::Display for HashError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Something went wrong in the hashing process!")
  }
}

pub type HashResult = Result<String, HashError>;
pub type HashCheckResult = Result<(bool, bool), HashError>;

pub trait IHashEncryptionService{  
   const SALT_SIZE: usize; 
   const KEY_SIZE: usize;
   fn new() -> Self;
   fn hash(input: &str) -> HashResult;
   fn check(hash: &str, input: &str) -> HashCheckResult;
}